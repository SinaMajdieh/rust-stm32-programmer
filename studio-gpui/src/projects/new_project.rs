use backend::project::Project;
use firmware_targets::{TargetKind, TemplateKind};
use gpui_kit::{
    base::{NavMotion, NavStackState, StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, IndexPath,
        alert::Alert,
        button::{Button, ButtonVariants},
        form::{Field, field, v_form},
        input::{Input, InputState},
        label::Label,
        select::{Select, SelectItem, SelectState},
    },
    prelude::*,
    *,
};
use std::path::PathBuf;
use strum::IntoEnumIterator;

use crate::projects::services::create_project;

pub struct NewProject {
    stack: WeakEntity<NavStackState>,
    name: Entity<InputState>,
    location: Entity<InputState>,
    target: Entity<SelectState<Vec<TargetOption>>>,
    template: Entity<SelectState<Vec<TemplateOption>>>,
    error: Option<String>,
}

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
    pub fn new(
        stack: WeakEntity<NavStackState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
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
            stack,
            name,
            location,
            target,
            template,
            error: None,
        }
    }

    fn browse(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let entity = cx.entity().downgrade();
        window
            .spawn(cx, async move |cx| {
                let Some(path) = rfd::AsyncFileDialog::new()
                    .set_title("Choose project location")
                    .pick_folder()
                    .await
                else {
                    return;
                };

                let path = path.path().to_string_lossy().into_owned();
                let _ = cx.update(|window, cx| {
                    let _ = entity.update(cx, |this, cx| {
                        this.location
                            .update(cx, |input, cx| input.set_value(path, window, cx));
                        this.error = None;
                        cx.notify();
                    });
                });
            })
            .detach();
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let project_name = self.name.read(cx).value().trim().to_owned();
        let root = PathBuf::from(self.location.read(cx).value().as_str()).join(&project_name);

        let target = *self
            .target
            .read(cx)
            .selected_value()
            .expect("Target missing");
        let template = *self
            .template
            .read(cx)
            .selected_value()
            .expect("Template missing");

        let project = Project::new()
            .with_name(project_name)
            .with_root(root)
            .with_target(target)
            .with_template(template);

        create_project(project, self.stack.clone(), window, cx);
    }

    fn cancel(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let stack = self.stack.clone();
        let Some(stack) = stack.upgrade() else {
            return;
        };

        stack.update(cx, |stack, cx| {
            stack.pop(NavMotion::Immediate, cx);
        });
    }
}

impl Render for NewProject {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("new-project")
            .size_full()
            .items_center()
            .justify_center()
            .overflow_y_scroll()
            .child(
                v_flex()
                    .w_full()
                    .max_w_128()
                    .mx_auto()
                    .px_6()
                    .py_8()
                    .gap_8()
                    .child(self.render_header(cx))
                    .child(self.render_form(cx))
                    .child(self.render_error_section(cx))
                    .child(self.render_actions(cx)),
            )
    }
}

impl NewProject {
    fn render_header(&self, cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(Label::new("New Project").text_2xl().font_semibold())
            .child(
                Label::new("Create a new STM32 Studio project.")
                    .text_base()
                    .text_color(cx.theme().muted_foreground),
            )
    }

    fn render_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_form()
            .gap_5()
            .child(field().label("Project Name").child(Input::new(&self.name)))
            .child(self.render_location_field(cx))
            .child(self.render_firmware_config(cx))
    }

    fn render_location_field(&self, cx: &mut Context<Self>) -> Field {
        field().label("Location").child(
            h_flex()
                .gap_2()
                .w_full()
                .child(div().flex_1().min_w_0().child(Input::new(&self.location)))
                .child(
                    Button::new("browse")
                        .label("Browse")
                        .on_click(cx.listener(|this, _, window, cx| this.browse(window, cx))),
                ),
        )
    }

    fn render_firmware_config(&self, cx: &mut Context<Self>) -> Field {
        field()
            .label("Firmware Configuration")
            .description("Choose the target microcontroller and firmware software stack.")
            .child(
                h_flex()
                    .w_full()
                    .gap_0()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .overflow_hidden()
                    .child(self.render_select_group("Firmware Target", &self.target, cx, true))
                    .child(self.render_select_group("Template", &self.template, cx, false)),
            )
    }

    fn render_select_group<T: SelectItem + 'static>(
        &self,
        label: &str,
        state: &Entity<SelectState<Vec<T>>>,
        cx: &Context<Self>,
        has_border_right: bool,
    ) -> impl IntoElement {
        let mut container = v_flex().flex_1().gap_2().p_3();

        if has_border_right {
            container = container.border_r_1().border_color(cx.theme().border);
        }

        container
            .child(
                Label::new(label)
                    .text_sm()
                    .text_color(cx.theme().muted_foreground),
            )
            .child(Select::new(state).w_full().appearance(false))
    }

    fn render_error_section(&self, _cx: &Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h(px(56.0))
            .when_some(self.error.clone(), |element, error| {
                element.child(Alert::error("project-validation-error", error))
            })
    }

    fn render_actions(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .justify_end()
            .gap_2()
            .mt_6()
            .child(
                Button::new("cancel")
                    .label("Cancel")
                    .on_click(cx.listener(|this, _, window, cx| this.cancel(window, cx))),
            )
            .child(
                Button::new("create")
                    .primary()
                    .label("Create Project")
                    .on_click(cx.listener(|this, _, window, cx| this.submit(window, cx))),
            )
    }
}
