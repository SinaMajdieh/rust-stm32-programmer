use backend::project::Project;
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
    base::StyledExt, div,
};

use super::{EditorStepper, Stage, StepperEvent};

#[derive(Debug)]
pub struct Editor {
    project: Entity<Project>,
    stepper: Entity<EditorStepper>,

    _subscriptions: Subscription,
}

impl Editor {
    pub fn new(project: Project, cx: &mut Context<Self>) -> Self {
        let project = cx.new(|_| project);

        let stepper = cx.new(|cx| EditorStepper::new(&project, cx));

        let stepper_subscription =
            cx.subscribe(&stepper, |this, _stepper, event, _cx| match event {
                StepperEvent::StageSelected(stage) => {
                    this.select_stage(*stage);
                }
            });

        Self {
            project,
            stepper,
            _subscriptions: stepper_subscription,
        }
    }

    // ---------------------------------------------------------------------
    // Workflow
    // ---------------------------------------------------------------------

    fn select_stage(&mut self, stage: Stage) {
        // The EditorStepper has already verified that the stage is available.
        //
        // The editor only needs to react to the selection here.
        println!("Selected stage: {stage:?}");
    }

    // ---------------------------------------------------------------------
    // Accessors
    // ---------------------------------------------------------------------

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    // ---------------------------------------------------------------------
    // Rendering
    // ---------------------------------------------------------------------

    fn render_stage(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.stepper.read(cx).current_stage() {
            Stage::Generation => div().size_full().child("Generation"),
            Stage::Build => div().size_full().child("Build"),
            Stage::Deploy => div().size_full().child("Deploy"),
        }
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .child(self.stepper.clone())
            .child(self.render_stage(cx))
    }
}
