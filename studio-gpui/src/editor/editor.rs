use backend::{
    GenerationError, LlmGenerator, ModelId,
    project::{GenerationRequest, Project},
};
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    assets::IconName, base::StyledExt, component::accordion::Accordion, div,
};

use crate::{
    TokioRuntime,
    editor::{
        build::{Build, BuildEvent},
        deploy::{Deploy, DeployEvent},
        generation::{Generation, GenerationEvent},
        stage::StageView,
    },
    settings::Settings,
};

/// Coordinates the project's generation, build, and deployment stages.
#[derive(Debug)]
pub struct Editor {
    project: Entity<Project>,
    generation: Entity<Generation>,
    build: Entity<Build>,
    deploy: Entity<Deploy>,
    _subscriptions: Vec<Subscription>,
}

impl Editor {
    /// Creates the editor and subscribes to stage events.
    pub fn new(project: Project, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project = cx.new(|_| project);
        let generation = cx.new(|cx| Generation::new(window, cx));
        let build = cx.new(|cx| Build::new(window, cx));
        let deploy = cx.new(|cx| Deploy::new(window, cx));

        let generation_subscription = cx.subscribe(
            &generation,
            |this, generation, event: &GenerationEvent, cx| match event {
                GenerationEvent::Submit { prompt, model } => {
                    this.handle_generate(prompt.clone(), model.clone(), generation, cx);
                }
            },
        );

        let build_subscription =
            cx.subscribe(&build, |this, build, event: &BuildEvent, cx| match event {
                BuildEvent::Submit => this.handle_build(build, cx),
            });

        let deploy_subscription =
            cx.subscribe(
                &deploy,
                |this, deploy, event: &DeployEvent, cx| match event {
                    DeployEvent::Submit => this.handle_deploy(deploy, cx),
                },
            );

        let mut editor = Self {
            project,
            generation,
            build,
            deploy,
            _subscriptions: vec![
                generation_subscription,
                build_subscription,
                deploy_subscription,
            ],
        };

        editor.sync_stage_states(cx);
        editor
    }

    /// Synchronizes non-busy stage states with the project's current artifacts.
    ///
    /// Busy states belong to in-flight operations, so reconciliation must not
    /// overwrite them. Failed states are transient and are replaced by the
    /// project-derived state the next time reconciliation runs.
    fn sync_stage_states(&mut self, cx: &mut Context<Self>) {
        let (has_generation, has_valid_build, has_valid_program) = {
            let project = self.project.read(cx);
            (
                project.has_generation(),
                project.has_valid_build(),
                project.has_valid_program(),
            )
        };

        self.generation.update(cx, |stage, cx| {
            if stage.state().is_busy() {
                return;
            }

            if has_generation {
                stage.mark_success(cx);
            } else {
                stage.mark_ready(cx);
            }
        });

        self.build.update(cx, |stage, cx| {
            if stage.state().is_busy() {
                return;
            }

            match (has_generation, has_valid_build) {
                (false, _) => stage.mark_stale(cx),
                (true, true) => stage.mark_success(cx),
                (true, false) => stage.mark_ready(cx),
            }
        });

        self.deploy.update(cx, |stage, cx| {
            if stage.state().is_busy() {
                return;
            }

            match (has_valid_build, has_valid_program) {
                (false, _) => stage.mark_stale(cx),
                (true, true) => stage.mark_success(cx),
                (true, false) => stage.mark_ready(cx),
            }
        });
    }

    /// Generates code and reconciles all stage states with the project.
    fn handle_generate(
        &mut self,
        prompt: String,
        model: ModelId,
        generation: Entity<Generation>,
        cx: &mut Context<Self>,
    ) {
        let settings = cx.global::<Settings>();

        let system_prompt = match settings.config.llm.system_prompt() {
            Ok(prompt) => prompt,
            Err(error) => {
                generation.update(cx, |stage, cx| {
                    stage.mark_failed(format!("Unable to load the LLM system prompt: {error}"), cx);
                });
                return;
            }
        };

        let request = GenerationRequest::new(model.as_str(), prompt, Some(system_prompt));
        let generator_config = settings.config.llm.generator.clone();
        let tokio = cx.global::<TokioRuntime>().handle().clone();

        let project = self.project.clone();
        self.build.update(cx, |stage, cx| stage.mark_stale(cx));
        self.deploy.update(cx, |stage, cx| stage.mark_stale(cx));

        cx.spawn(async move |this, cx| {
            let result = tokio
                .spawn(async move {
                    let generator = LlmGenerator::from_config(generator_config)?;
                    let output = Project::generate_output(&request, &generator).await?;
                    Ok::<_, GenerationError>((request, output))
                })
                .await;

            let (request, output) = match result {
                Ok(Ok(result)) => result,
                Ok(Err(error)) => {
                    generation.update(cx, |stage, cx| {
                        stage.mark_failed(error.to_string(), cx);
                    });
                    return;
                }
                Err(error) => {
                    generation.update(cx, |stage, cx| {
                        stage.mark_failed(format!("Generation task failed: {error}"), cx);
                    });
                    return;
                }
            };

            let result = project.update(cx, |project, cx| {
                project.apply_generation(request, output);
                let save_result = project.save();
                cx.notify();
                save_result
            });

            // Reconcile even when saving fails: the generated artifact has
            // already changed in memory, invalidating downstream artifacts.
            if result.is_ok() {
                let _ = this.update(cx, |editor, cx| {
                    editor.sync_stage_states(cx);
                });
            }

            match result {
                Ok(()) => {
                    generation.update(cx, |stage, cx| stage.mark_success(cx));
                }
                Err(error) => {
                    generation.update(cx, |stage, cx| {
                        stage.mark_failed(
                            format!("Code was generated, but saving failed: {error}"),
                            cx,
                        );
                    });
                }
            }
        })
        .detach();
    }

    /// Builds the generated code and reconciles stage states.
    fn handle_build(&mut self, build: Entity<Build>, cx: &mut Context<Self>) {
        let project = self.project.clone();
                self.deploy.update(cx, |stage, cx| stage.mark_stale(cx));

        let input = match project.read(cx).prepare_build() {
            Ok(input) => input,
            Err(error) => {
                build.update(cx, |stage, cx| {
                    stage.mark_failed(format!("Build preparation failed: {error}"), cx);
                });
                return;
            }
        };

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { input.compile() })
                .await;

            let output = match result {
                Ok(output) => output,
                Err(error) => {
                    build.update(cx, |stage, cx| {
                        stage.mark_failed(format!("Build failed: {error}"), cx);
                    });
                    return;
                }
            };

            let result = project.update(cx, |project, cx| {
                project.apply_build(output);
                let save_result = project.save();
                cx.notify();
                save_result
            });

            if result.is_ok() {
                let _ = this.update(cx, |editor, cx| {
                    editor.sync_stage_states(cx);
                });
            }

            match result {
                Ok(()) => {
                    build.update(cx, |stage, cx| stage.mark_success(cx));
                }
                Err(error) => {
                    build.update(cx, |stage, cx| {
                        stage.mark_failed(
                            format!("Compilation succeeded, but saving failed: {error}"),
                            cx,
                        );
                    });
                }
            }
        })
        .detach();
    }

    /// Programs the board with the compiled firmware and reconciles stage states.
    fn handle_deploy(&mut self, deploy: Entity<Deploy>, cx: &mut Context<Self>) {
        let project = self.project.clone();

        let input = match project.read(cx).prepare_program() {
            Ok(input) => input,
            Err(error) => {
                deploy.update(cx, |stage, cx| {
                    stage.mark_failed(format!("Deployment preparation failed: {error}"), cx);
                });
                return;
            }
        };

        cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { input.execute() })
                .await;

            let output = match result {
                Ok(output) => output,
                Err(error) => {
                    deploy.update(cx, |stage, cx| {
                        stage.mark_failed(format!("Deployment failed: {error}"), cx);
                    });
                    return;
                }
            };

            let result = project.update(cx, |project, cx| {
                project.apply_program(output);
                let save_result = project.save();
                cx.notify();
                save_result
            });

            if result.is_ok() {
                let _ = this.update(cx, |editor, cx| {
                    editor.sync_stage_states(cx);
                });
            }

            match result {
                Ok(()) => {
                    deploy.update(cx, |stage, cx| stage.mark_success(cx));
                }
                Err(error) => {
                    deploy.update(cx, |stage, cx| {
                        stage.mark_failed(
                            format!("Deployment succeeded, but saving failed: {error}"),
                            cx,
                        );
                    });
                }
            }
        })
        .detach();
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let generation_stale = self.generation.read(cx).state().is_stale();
        let build_stale = self.build.read(cx).state().is_stale();
        let deploy_stale = self.deploy.read(cx).state().is_stale();

        div()
            .size_full()
            .v_flex()
            .items_center()
            .justify_center()
            .p_8()
            .child(
                div().w_full().max_w_2_3().child(
                    Accordion::new("editor-accordion")
                        .multiple(true)
                        .rounded_xl()
                        .bordered(false)
                        .item(|item| {
                            item.icon(IconName::Sparkles)
                                .title("Generation")
                                .open(!generation_stale)
                                .disabled(generation_stale)
                                .border_0()
                                .child(self.generation.clone())
                        })
                        .item(|item| {
                            item.icon(IconName::Cpu)
                                .title("Build")
                                .open(!build_stale)
                                .disabled(build_stale)
                                .border_0()
                                .child(self.build.clone())
                        })
                        .item(|item| {
                            item.icon(IconName::Upload)
                                .title("Deploy")
                                .open(!deploy_stale)
                                .disabled(deploy_stale)
                                .border_0()
                                .child(self.deploy.clone())
                        }),
                ),
            )
    }
}
