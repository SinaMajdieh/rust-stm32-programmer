use backend::{
    GenerationError, LlmGenerator, ModelId,
    project::{GenerationRequest, Project},
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
    editor::{Generation, GenerationEvent},
    settings::Settings,
};

use super::{EditorStepper, Stage, StepperEvent};

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
        Self {
            project,
            generation,
            active_stage: Some(Stage::Generation),
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

    fn handle_generate(
        &mut self,
        prompt: String,
        model: ModelId,
        generation: Entity<Generation>,
        cx: &mut Context<Self>,
    ) {
        let settings = cx.global::<Settings>();

        let system_prompt = settings.config.llm.system_prompt().unwrap();

        let request = GenerationRequest::new(model.as_str(), prompt, Some(system_prompt));

        let generator_config = settings.config.llm.generator.clone();
        let project = self.project.clone();

        generation.update(cx, |generation, cx| {
            generation.set_loading(true, cx);
        });

        cx.spawn(async move |this, cx| {
            let result = async {
                let generator = LlmGenerator::from_config(generator_config)?;

                let output = Project::generate_output(&request, &generator).await?;

                project.update(cx, |project, cx| {
                    project.apply_generation(request, output);
                    cx.notify();
                });

                Ok::<_, GenerationError>(())
            }
            .await;

            match result {
                Ok(()) => {
                    generation.update(cx, |generation, cx| {
                        generation.set_loading(false, cx);
                    });

                    this.update(cx, |editor, cx| {
                        editor.active_stage = Some(Stage::Build);
                        cx.notify();
                    });
                }

                Err(error) => {
                    generation.update(cx, |generation, cx| {
                        generation.set_loading(false, cx);
                        eprintln!("{}", error)
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
                                .open(active_stage == Some(Stage::Build))
                                .child("Build will be here soon")
                        })
                        .item(|item| {
                            item.icon(IconName::Upload)
                                .title("Deploy")
                                .open(active_stage == Some(Stage::Deploy))
                                .child("Deploy will be here soon")
                        }),
                ),
            )
    }
}
