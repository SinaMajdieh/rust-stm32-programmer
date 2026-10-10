use backend::{Model, ModelId};
use gpui_kit::{
    AppContext, Context, Entity, EventEmitter, IntoElement, ParentElement, Render, SharedString,
    Styled, Window,
    base::{Disableable, IndexPath, StyledExt, input::TextareaState},
    component::{
        ActiveTheme,
        alert::Alert,
        button::{Button, ButtonVariants},
        input::Textarea,
        select::{SearchableVec, Select, SelectItem, SelectState},
        shimmer::ShimmerText,
    },
    div, rems,
};

use crate::{
    editor::stage::{StageState, StageView},
    settings::Settings,
};

/// Adapter around a backend model for use in GPUI-Kit's select component.
#[derive(Clone, Debug)]
pub struct ModelOption(Model);

impl From<Model> for ModelOption {
    fn from(model: Model) -> Self {
        Self(model)
    }
}

impl SelectItem for ModelOption {
    type Value = Model;

    fn title(&self) -> SharedString {
        self.0.name().into()
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

/// Events emitted by the generation form.
pub enum GenerationEvent {
    /// Requests code generation using the supplied prompt and model.
    Submit { prompt: String, model: ModelId },
}

/// Form for submitting code-generation requests.
///
/// The parent controller owns the generation workflow and updates the stage
/// state as asynchronous work progresses.
pub struct Generation {
    prompt: Entity<TextareaState>,
    models: Entity<SelectState<SearchableVec<ModelOption>>>,
    state: StageState,
}

impl EventEmitter<GenerationEvent> for Generation {}

impl StageView for Generation {
    fn state(&self) -> &StageState {
        &self.state
    }

    fn update_state(&mut self, state: StageState) {
        self.state = state;
    }

    fn busy_message() -> &'static str {
        "Generating code..."
    }

    fn success_message() -> &'static str {
        "Code generated successfully."
    }
}

impl Generation {
    /// Creates the generation form using the application's current settings.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let llm = &cx.global::<Settings>().config.llm;

        let selected_index = llm
            .generator
            .available_models
            .iter()
            .position(|model| model.id() == llm.selected_model)
            .map(IndexPath::new);

        let model_options = llm
            .generator
            .available_models
            .iter()
            .cloned()
            .map(ModelOption::from)
            .collect::<Vec<_>>();

        let models = cx.new(|cx| {
            SelectState::new(
                SearchableVec::from(model_options),
                selected_index,
                window,
                cx,
            )
            .searchable(true)
        });

        let prompt = cx.new(|cx| {
            TextareaState::new(window, cx)
                .auto_grow(2, 8)
                .placeholder("What would you like to build?")
                .default_value("Make the onboard LED blink every 2 seconds.")
        });

        Self {
            prompt,
            models,
            state: StageState::Ready,
        }
    }

    /// Validates the form and emits a generation request.
    fn generate(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.state.is_busy() {
            return;
        }

        let prompt = self.prompt.read(cx).text().to_string().trim().to_owned();

        if prompt.is_empty() {
            return;
        }

        let Some(model) = self.models.read(cx).selected_value().map(Model::id) else {
            return;
        };

        self.mark_busy(cx);
        cx.emit(GenerationEvent::Submit { prompt, model });
    }

    /// Renders the current generation status, if it has a visible message.
    fn render_status(&self) -> Option<impl IntoElement> {
        match &self.state {
            StageState::Busy(message) => Some(
                div()
                    .w_full()
                    .h_flex()
                    .items_center()
                    .child(ShimmerText::new(message.clone()).text_lg().font_medium())
                    .into_any_element(),
            ),

            StageState::Succeeded(message) => Some(
                div()
                    .w_full()
                    .child(
                        Alert::success("generation-success", message.clone())
                            .title("Generation Completed"),
                    )
                    .into_any_element(),
            ),

            StageState::Failed(message) => Some(
                div()
                    .w_full()
                    .child(
                        Alert::error("generation-error", message.clone())
                            .title("Generation Failed"),
                    )
                    .into_any_element(),
            ),

            StageState::Stale | StageState::Ready => None,
        }
    }
}

impl Render for Generation {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let is_busy = self.state.is_busy();

        div()
            .size_full()
            .v_flex()
            .p_4()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w_full()
                    .max_w_5_6()
                    .v_flex()
                    .gap_4()
                    .child(
                        div()
                            .font_semibold()
                            .text_xl()
                            .text_color(theme.foreground)
                            .child("What would you like to build?"),
                    )
                    .child(
                        div()
                            .w_full()
                            .rounded_lg()
                            .border_1()
                            .border_color(theme.border)
                            .bg(theme.colors.secondary)
                            .v_flex()
                            .overflow_hidden()
                            .child(
                                Textarea::new(&self.prompt)
                                    .bg(theme.colors.secondary)
                                    .bordered(false)
                                    .text_lg(),
                            )
                            .child(
                                div()
                                    .h_flex()
                                    .items_center()
                                    .justify_end()
                                    .gap_2()
                                    .px_2()
                                    .py_2()
                                    .child(
                                        div().w_64().flex_none().child(
                                            Select::new(&self.models)
                                                .w_full()
                                                .bg(theme.colors.secondary)
                                                .border_0()
                                                .rounded_full()
                                                .title_prefix("Model: ")
                                                .menu_max_h(rems(10.0)),
                                        ),
                                    )
                                    .child(
                                        Button::new("generate")
                                            .primary()
                                            .label("Generate")
                                            .disabled(is_busy)
                                            .rounded_full()
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.generate(window, cx);
                                            })),
                                    ),
                            ),
                    )
                    .children(self.render_status()),
            )
    }
}
