use backend::{
    GenerationError, GenerationOutput, LlmGenerator, ModelId,
    project::{GenerationRequest, Project, Stage},
};
use gpui_kit::{
    AppContext, Context, Entity, FontWeight, InteractiveElement, IntoElement, ParentElement,
    Render, Styled, Subscription, Window,
    assets::IconName,
    base::{StyledExt, h_flex},
    component::{ActiveTheme, Colorize, accordion::Accordion, green_500, green_600},
    div,
    private::anyhow,
    px, rgb,
};

use crate::{
    TokioRuntime,
    editor::{Generation, GenerationEvent},
    settings::Settings,
};

#[derive(Debug)]
pub struct Editor {
    project: Entity<Project>,
    generation: Entity<Generation>,
    active_stage: Option<Stage>,
    _subscriptions: Vec<Subscription>,
}

impl Editor {
    pub fn new(project: Project, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project = cx.new(|_| project);
        let generation = cx.new(|cx| Generation::new(window, cx));

        let sub = cx.subscribe(
            &generation,
            |this, generation, event: &GenerationEvent, cx| match event {
                GenerationEvent::Submit { prompt, model } => {
                    this.handle_generate(prompt.to_owned(), model.to_owned(), generation, cx);
                }
            },
        );

        let active_stage = Some(project.read(cx).latest_stage().into());

        Self {
            project,
            generation,
            active_stage,
            _subscriptions: vec![sub],
        }
    }

    fn select_stage(&mut self, stage: Stage) {
        // The EditorStepper has already verified that the stage is available.
        //
        // The editor only needs to react to the selection here.
        println!("Selected stage: {stage:?}");
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    /// Handles a generation submission from the [`Generation`] view.
    ///
    /// Spawns the LLM generation on the Tokio runtime and reports the outcome
    /// back to the view via its success/failure feedback alerts.
    fn handle_generate(
        &mut self,
        prompt: String,
        model: ModelId,
        generation: Entity<Generation>,
        cx: &mut Context<Self>,
    ) {
        let settings = cx.global::<Settings>();

        // Guard against missing settings early to avoid panics.
        let Ok(system_prompt) = settings.config.llm.system_prompt() else {
            generation.update(cx, |generation, cx| {
                generation.failed("Missing LLM system prompt in settings.", cx);
            });
            return;
        };

        let request = GenerationRequest::new(model.as_str(), prompt, Some(system_prompt));
        let generator_config = settings.config.llm.generator.clone();
        let project = self.project.clone();
        let tokio = cx.global::<TokioRuntime>().handle().clone();

        generation.update(cx, |generation, cx| {
            generation.set_loading(true, cx);
        });

        cx.spawn(async move |this, cx| {
            let result = tokio
                .spawn(async move {
                    let generator = LlmGenerator::from_config(generator_config)?;
                    let output = Project::generate_output(&request, &generator).await?;

                    // Pass `request` back with the output to avoid cloning or borrow issues
                    Ok::<_, GenerationError>((request, output))
                })
                .await;

            match result {
                Ok(Ok((request, output))) => {
                    // Apply and persist without unwrapping.
                    let save_result = project.update(cx, |project, cx| {
                        project.apply_generation(request, output);
                        cx.notify();
                        project.save()
                    });

                    match save_result {
                        Ok(()) => {
                            generation.update(cx, |generation, cx| {
                                generation.success("Code generated successfully.", cx);
                            });

                            this.update(cx, |editor, cx| {
                                editor.active_stage = Some(Stage::Build);
                                cx.notify();
                            });
                        }
                        Err(error) => {
                            generation.update(cx, |generation, cx| {
                                generation
                                    .failed(format!("Generated, but saving failed: {error}"), cx);
                            });
                        }
                    }
                }

                Ok(Err(error)) => {
                    generation.update(cx, |generation, cx| {
                        generation.failed(error.to_string(), cx);
                    });
                }

                Err(error) => {
                    generation.update(cx, |generation, cx| {
                        generation.failed(format!("Generation task failed: {error}"), cx);
                    });
                }
            }
        })
        .detach();
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_stage = self.active_stage;

        div()
            .size_full()
            .v_flex()
            .items_center()
            .justify_center()
            .p_8()
            .child(
                div().w_full().max_w_2_3().child(
                    Accordion::new("editor-accordion")
                        .rounded_xl()
                        .bordered(false)
                        .on_toggle_click(cx.listener(|this, indices: &[usize], _window, cx| {
                            this.active_stage = indices
                                .first()
                                .copied()
                                .and_then(|idx| Stage::try_from(idx).ok());

                            cx.notify();
                        }))
                        .item(|item| {
                            item.icon(IconName::Sparkles)
                                .title("Generation")
                                .open(active_stage == Some(Stage::Generation))
                                .child(self.generation.clone())
                        })
                        .item(|item| {
                            item.icon(IconName::Cpu)
                                .title("Build")
                                .disabled(!self.project().read(cx).has_generation())
                                .open(active_stage == Some(Stage::Build))
                                .child("Build will be here soon")
                        })
                        .item(|item| {
                            item.icon(IconName::Upload)
                                .title("Deploy")
                                .disabled(!self.project().read(cx).has_valid_build())
                                .open(active_stage == Some(Stage::Deploy))
                                .child("Deploy will be here soon")
                        }),
                ),
            )
    }
}
