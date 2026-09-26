use backend::project::Project;
use gpui_kit::{
    Context, Entity, EventEmitter, IntoElement, Render, Subscription, Window,
    assets::IconName,
    base::v_flex,
    component::{
        Sizable,
        stepper::{Stepper, StepperItem},
    },
    div,
    prelude::*,
};

use super::stage::Stage;

// -------------------------------------------------------------------------
// Stage availability
// -------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StageAvailability {
    generation: bool,
    build: bool,
    deploy: bool,
}

impl Default for StageAvailability {
    fn default() -> Self {
        Self {
            generation: true,
            build: false,
            deploy: false,
        }
    }
}

impl StageAvailability {
    pub fn from_project(project: &Project) -> Self {
        Self {
            generation: true,
            build: project.has_generation(),
            deploy: project.has_valid_build(),
        }
    }

    pub const fn is_enabled(self, stage: Stage) -> bool {
        match stage {
            Stage::Generation => self.generation,
            Stage::Build => self.build,
            Stage::Deploy => self.deploy,
        }
    }

    pub const fn fallback_stage(self) -> Stage {
        if self.deploy {
            Stage::Deploy
        } else if self.build {
            Stage::Build
        } else {
            Stage::Generation
        }
    }
}

// -------------------------------------------------------------------------
// Events
// -------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepperEvent {
    StageSelected(Stage),
}

// -------------------------------------------------------------------------
// Editor stepper
// -------------------------------------------------------------------------

#[derive(Debug)]
pub struct EditorStepper {
    current_stage: Stage,
    availability: StageAvailability,

    _project_subscription: Subscription,
}

impl EventEmitter<StepperEvent> for EditorStepper {}

impl EditorStepper {
    pub fn new(project: &Entity<Project>, cx: &mut Context<Self>) -> Self {
        let availability = {
            let project = project.read(cx);
            StageAvailability::from_project(&project)
        };

        let project_subscription = cx.observe(&project, |this, project, cx| {
            let availability = {
                let project = project.read(cx);
                StageAvailability::from_project(&project)
            };

            this.set_availability(availability, cx);
        });

        Self {
            current_stage: Stage::Generation,
            availability,
            _project_subscription: project_subscription,
        }
    }

    // ---------------------------------------------------------------------
    // State
    // ---------------------------------------------------------------------

    pub fn current_stage(&self) -> Stage {
        self.current_stage
    }

    pub fn availability(&self) -> StageAvailability {
        self.availability
    }

    // ---------------------------------------------------------------------
    // Project observation
    // ---------------------------------------------------------------------

    fn set_availability(&mut self, availability: StageAvailability, cx: &mut Context<Self>) {
        let current_stage = if availability.is_enabled(self.current_stage) {
            self.current_stage
        } else {
            availability.fallback_stage()
        };

        if self.availability == availability && self.current_stage == current_stage {
            return;
        }

        self.availability = availability;
        self.current_stage = current_stage;

        cx.notify();
    }

    // ---------------------------------------------------------------------
    // Interaction
    // ---------------------------------------------------------------------

    fn select_stage(&mut self, stage: Stage, cx: &mut Context<Self>) {
        if !self.availability.is_enabled(stage) {
            return;
        }

        if self.current_stage == stage {
            return;
        }

        self.current_stage = stage;

        cx.emit(StepperEvent::StageSelected(stage));
        cx.notify();
    }
}

// -------------------------------------------------------------------------
// Rendering
// -------------------------------------------------------------------------

impl Render for EditorStepper {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_stage = self.current_stage;
        let availability = self.availability;

        div().p_12().child(
            Stepper::new("editor-stepper")
                .large()
                .text_center(true)
                .selected_index(current_stage.index())
                .items([
                    StepperItem::new()
                        .disabled(!availability.is_enabled(Stage::Generation))
                        .icon(IconName::Sparkles)
                        .text_3xl()
                        .child(
                            v_flex()
                                .items_center()
                                .child("Generate")
                                .child("Generate code for the firmware"),
                        ),
                    StepperItem::new()
                        .disabled(!availability.is_enabled(Stage::Build))
                        .icon(IconName::Cpu)
                        .child(
                            v_flex()
                                .items_center()
                                .child("Build")
                                .child("Build the project"),
                        ),
                    StepperItem::new()
                        .disabled(!availability.is_enabled(Stage::Deploy))
                        .icon(IconName::Upload)
                        .child(
                            v_flex()
                                .items_center()
                                .child("Deploy")
                                .child("Deploy the firmware"),
                        ),
                ])
                .on_click(cx.listener(|this, index, _, cx| {
                    let Ok(stage) = Stage::try_from(*index) else {
                        return;
                    };

                    this.select_stage(stage, cx);
                })),
        )
    }
}
