use backend::{Model, ModelId};
use gpui_kit::{
    AppContext, Context, Entity, EventEmitter, InteractiveElement, IntoElement, ParentElement,
    Render, SharedString, Styled, Window,
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

use crate::settings::Settings;

/// Adapter wrapper around backend `Model` to implement GPUI-Kit's `SelectItem`.
#[derive(Clone, Debug)]
pub struct ModelOption(pub Model);

impl SelectItem for ModelOption {
    type Value = Model;

    #[inline]
    fn title(&self) -> SharedString {
        self.0.name().into()
    }

    #[inline]
    fn value(&self) -> &Self::Value {
        &self.0
    }
}

/// Represents the status message rendered below the input controls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenerationFeedback {
    Success(SharedString),
    Error(SharedString),
}

/// Events dispatched by the `Generation` view to notify parent controllers.
pub enum GenerationEvent {
    Submit { prompt: String, model: ModelId },
}

/// Generation form component managing user prompts, model selection,
/// loading states, and result alerts.
pub struct Generation {
    state: Entity<TextareaState>,
    models: Entity<SelectState<SearchableVec<ModelOption>>>,
    is_loading: bool,
    feedback: Option<GenerationFeedback>,
}

impl EventEmitter<GenerationEvent> for Generation {}

impl Generation {
    /// Constructs a new `Generation` view and initializes inner form states.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = cx.global::<Settings>();
        let llm_config = &settings.config.llm;

        // Locate currently configured model index, if present.
        let selected_index = llm_config
            .generator
            .available_models
            .iter()
            .position(|m| m.id() == llm_config.selected_model)
            .map(IndexPath::new);

        let model_options: Vec<ModelOption> = llm_config
            .generator
            .available_models
            .iter()
            .cloned()
            .map(ModelOption)
            .collect();

        let models = cx.new(|cx| {
            SelectState::new(
                SearchableVec::from(model_options),
                selected_index,
                window,
                cx,
            )
            .searchable(true)
        });

        let state = cx.new(|cx| {
            TextareaState::new(window, cx)
                .auto_grow(4, 8)
                .placeholder("What would you like to build?")
                .default_value("Make the onboard LED blink every 2 seconds.")
        });

        Self {
            state,
            models,
            is_loading: false,
            feedback: None,
        }
    }

    /// Explicitly updates the loading indicator state.
    pub fn set_loading(&mut self, loading: bool, cx: &mut Context<Self>) {
        if self.is_loading != loading {
            self.is_loading = loading;
            cx.notify();
        }
    }

    /// Halts loading and displays a success alert message.
    pub fn success(&mut self, message: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.is_loading = false;
        self.feedback = Some(GenerationFeedback::Success(message.into()));
        cx.notify();
    }

    /// Halts loading and displays an error alert with the failure description.
    pub fn failed(&mut self, error: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.is_loading = false;
        self.feedback = Some(GenerationFeedback::Error(error.into()));
        cx.notify();
    }

    /// Clears any active alert banner.
    pub fn clear_feedback(&mut self, cx: &mut Context<Self>) {
        if self.feedback.is_some() {
            self.feedback = None;
            cx.notify();
        }
    }

    /// Validates inputs, resets previous alerts, and emits `GenerationEvent::Submit`.
    fn generate(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.is_loading {
            return;
        }

        // Borrow textarea text and trim without redundant heap reallocations.
        let raw_text = self.state.read(cx).text().to_string();
        let prompt = raw_text.trim();
        if prompt.is_empty() {
            return;
        }

        let Some(model) = self.models.read(cx).selected_value().map(|m| m.id()) else {
            return;
        };

        let prompt = prompt.to_owned();

        // Clear previous notifications & set progress state
        self.feedback = None;
        self.is_loading = true;
        cx.notify();

        cx.emit(GenerationEvent::Submit { prompt, model });
    }

    /// Builds the status container rendered directly below the main card.
    fn render_status(&self, cx: &Context<Self>) -> Option<impl IntoElement> {
        if self.is_loading {
            return Some(
                div()
                    .w_full()
                    .h_flex()
                    .items_center()
                    .child(
                        ShimmerText::new("Generating code...")
                            .text_lg()
                            .font_medium(),
                    )
                    .into_any_element(),
            );
        }

        if let Some(feedback) = &self.feedback {
            let alert_element = match feedback {
                GenerationFeedback::Success(msg) => {
                    Alert::success("gen-success", msg.clone()).title("Generation Completed")
                }
                GenerationFeedback::Error(err) => {
                    Alert::error("gen-error", err.clone()).title("Generation Failed")
                }
            };

            return Some(div().w_full().child(alert_element).into_any_element());
        }

        None
    }
}

impl Render for Generation {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div().size_full().v_flex().items_center().p_8().child(
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
                            Textarea::new(&self.state)
                                .bg(theme.colors.secondary)
                                .bordered(false)
                                .text_lg()
                                .min_h(rems(6.0)),
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
                                        .disabled(self.is_loading)
                                        .rounded_full()
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.generate(window, cx)
                                        })),
                                ),
                        ),
                )
                .children(self.render_status(cx)),
        )
    }
}
