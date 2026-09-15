use backend::project::Project;
use firmware_targets::{TargetKind, TemplateKind};

use gpui_kit::{
    base::{StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, IndexPath,
        alert::Alert,
        button::{Button, ButtonVariants},
        form::{field, v_form},
        input::{Input, InputState},
        label::Label,
        select::{Select, SelectItem, SelectState},
    },
    prelude::*,
    *,
};

use std::path::PathBuf;
use strum::IntoEnumIterator;

pub enum NewProjectEvent {
    Cancel,
    Create(Project),
}

pub struct NewProject {
    name: Entity<InputState>,
    location: Entity<InputState>,
    target: Entity<SelectState<Vec<TargetOption>>>,
    template: Entity<SelectState<Vec<TemplateOption>>>,
    error: Option<String>,
}

impl EventEmitter<NewProjectEvent> for NewProject {}

#[derive(Clone)]
struct TargetOption(TargetKind);

impl SelectItem for TargetOption {
    type Value = TargetKind;

    fn title(&self) -> SharedString {
        self.0.to_string().into()
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

#[derive(Clone)]
struct TemplateOption(TemplateKind);

impl SelectItem for TemplateOption {
    type Value = TemplateKind;

    fn title(&self) -> SharedString {
        self.0.to_string().into()
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

impl NewProject {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx).default_value("MyProject"));

        let location =
            cx.new(|cx| InputState::new(window, cx).placeholder("Choose project location..."));

        let target = cx.new(|cx| {
            SelectState::new(
                TargetKind::iter().map(TargetOption).collect(),
                Some(IndexPath::default()),
                window,
                cx,
            )
        });

        let template = cx.new(|cx| {
            SelectState::new(
                TemplateKind::iter().map(TemplateOption).collect(),
                Some(IndexPath::default()),
                window,
                cx,
            )
        });

        Self {
            name,
            location,
            target,
            template,
            error: None,
        }
    }

    fn browse(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = rfd::FileDialog::new()
            .set_title("Choose project location")
            .pick_folder()
        else {
            return;
        };

        let path = path.to_string_lossy().into_owned();

        self.location.update(cx, |input, cx| {
            input.set_value(path, window, cx);
        });

        self.error = None;
        cx.notify();
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let target = *self
            .target
            .read(cx)
            .selected_value()
            .expect("NewProject target select should always have a selection");

        let template = *self
            .template
            .read(cx)
            .selected_value()
            .expect("NewProject template should always have a selection");

        let project = Project::new()
            .with_name(self.name.read(cx).value().trim().to_owned())
            .with_root(PathBuf::from(self.location.read(cx).value().as_str()))
            .with_target(target)
            .with_template(template);
        // let request = Project {
        //     name: self.name.read(cx).value().trim().to_owned(),
        //     location: PathBuf::from(self.location.read(cx).value().as_str()),
        //     target,
        //     template,
        // };

        // if let Err(error) = request.validate() {
        //     self.error = Some(error);
        //     cx.notify();
        //     return;
        // }

        self.error = None;

        cx.emit(NewProjectEvent::Create(project));
    }
}

impl Render for NewProject {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let error = self.error.clone();

        v_flex()
            .id("new-project")
            .size_full()
            .overflow_y_scroll()
            .child(
                v_flex()
                    .w_full()
                    .max_w_128()
                    .mx_auto()
                    .px_6()
                    .py_8()
                    .gap_8()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("New Project")
                                    .text_2xl()
                                    .font_semibold(),
                            )
                            .child(
                                Label::new(
                                    "Create a new STM32 Studio project.",
                                )
                                .text_base()
                                .text_color(
                                    cx.theme().muted_foreground,
                                ),
                            ),
                    )
                    .child(
                        v_form()
                            .gap_5()
                            .child(
                                field()
                                    .label("Project Name")
                                    .child(Input::new(&self.name)),
                            )
                            .child(
                                field()
                                    .label("Location")
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .w_full()
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .min_w_0()
                                                    .child(
                                                        Input::new(
                                                            &self.location,
                                                        ),
                                                    ),
                                            )
                                            .child(
                                                Button::new("browse")
                                                    .label("Browse")
                                                    .on_click(
                                                        cx.listener(
                                                            |this,
                                                             _,
                                                             window,
                                                             cx| {
                                                                this.browse(
                                                                    window,
                                                                    cx,
                                                                );
                                                            },
                                                        ),
                                                    ),
                                            ),
                                    ),
                            )
                            .child(
                                field()
                                    .label(
                                        "Firmware Configuration",
                                    )
                                    .description(
                                        "Choose the target microcontroller and firmware software stack.",
                                    )
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .gap_0()
                                            .border_1()
                                            .border_color(
                                                cx.theme().border,
                                            )
                                            .rounded(
                                                cx.theme().radius,
                                            )
                                            .overflow_hidden()
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .gap_2()
                                                    .p_3()
                                                    .border_r_1()
                                                    .border_color(
                                                        cx.theme().border,
                                                    )
                                                    .child(
                                                        Label::new(
                                                            "Firmware Target",
                                                        )
                                                        .text_sm()
                                                        .text_color(
                                                            cx.theme()
                                                                .muted_foreground,
                                                        ),
                                                    )
                                                    .child(
                                                        Select::new(
                                                            &self.target,
                                                        )
                                                        .w_full()
                                                        .appearance(
                                                            false,
                                                        ),
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .gap_2()
                                                    .p_3()
                                                    .child(
                                                        Label::new(
                                                            "Template",
                                                        )
                                                        .text_sm()
                                                        .text_color(
                                                            cx.theme()
                                                                .muted_foreground,
                                                        ),
                                                    )
                                                    .child(
                                                        Select::new(
                                                            &self.template,
                                                        )
                                                        .w_full()
                                                        .appearance(
                                                            false,
                                                        ),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .w_full()
                            .h(px(56.0))
                            .when_some(
                                error,
                                |element, error| {
                                    element.child(
                                        Alert::error(
                                            "project-validation-error",
                                            error,
                                        ),
                                    )
                                },
                            ),
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .mt_6()
                            .child(
                                Button::new("cancel")
                                    .label("Cancel")
                                    .on_click(
                                        cx.listener(
                                            |_, _, _, cx| {
                                                cx.emit(
                                                    NewProjectEvent::Cancel,
                                                );
                                            },
                                        ),
                                    ),
                            )
                            .child(
                                Button::new("create")
                                    .primary()
                                    .label("Create Project")
                                    .on_click(
                                        cx.listener(
                                            |this, _, _, cx| {
                                                this.submit(cx);
                                            },
                                        ),
                                    ),
                            ),
                    ),
            )
    }
}
