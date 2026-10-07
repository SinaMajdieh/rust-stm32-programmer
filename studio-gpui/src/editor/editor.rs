use backend::project::Project;
use gpui_kit::{
    AppContext, Context, Entity, FontWeight, InteractiveElement, IntoElement, ParentElement,
    Render, Styled, Subscription, Window,
    assets::IconName,
    base::{StyledExt, h_flex},
    component::{ActiveTheme, accordion::Accordion, green_500, green_600},
    div, px, rgb,
};

use crate::editor::Generation;

use super::{EditorStepper, Stage, StepperEvent};

#[derive(Debug)]
pub struct Editor {
    project: Entity<Project>,
    generation: Entity<Generation>,
    active_stage: Option<Stage>,
}

impl Editor {
    pub fn new(project: Project, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project = cx.new(|_| project);
        let generation = cx.new(|cx| Generation::new(window, cx));
        Self {
            project,
            generation,
            active_stage: Some(Stage::Generation),
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
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_stage = self.active_stage;
        div().flex().justify_center().p_8().child(
            Accordion::new("editor-accordion")
                .max_w_5_6()
                .rounded_xl()
                .border_2()
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
        )
    }
}
