use backend::{Model, project::GenerationRequest};
use gpui_kit::{
    AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    SharedString, Styled, Window,
    WindowBackgroundAppearance::Transparent,
    base::{IndexPath, StyledExt, input::TextareaState},
    component::{
        ActiveTheme, Colorize,
        button::{Button, ButtonVariants},
        input::Textarea,
        select::{SearchableVec, Select, SelectItem, SelectState},
    },
    div, rems, transparent_black, transparent_white,
};

use crate::settings::Settings;

#[derive(Clone)]
pub struct ModelOption(pub Model);

impl SelectItem for ModelOption {
    type Value = Model;

    fn title(&self) -> SharedString {
        self.0.name().into()
    }

    fn value(&self) -> &Self::Value {
        &self.0
    }
}

pub struct Generation {
    state: Entity<TextareaState>,
    models: Entity<SelectState<SearchableVec<ModelOption>>>,
}

impl Generation {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = cx.global::<Settings>();
        let llm_config = &settings.config.llm;

        // Find the selected model index before allocating/wrapping
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
        });

        Self { state, models }
    }

    fn generate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let settings = cx.global::<Settings>();

        let prompt = self.state.read(cx).text().to_string();
        let prompt = prompt.trim();
        let model = self.models.read(cx).selected_value().cloned().unwrap().id();
        let system_prompt = settings.config.llm.system_prompt().unwrap();

        let request = GenerationRequest::new(model.as_str(), prompt, Some(system_prompt));
        println!("{:#?}", request);
    }
}

impl Render for Generation {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                        .text_color(cx.theme().foreground)
                        .child("What would you like to build?"),
                )
                .child(
                    div()
                        .w_full()
                        .rounded_lg()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().colors.secondary)
                        .v_flex()
                        .overflow_hidden()
                        .child(
                            Textarea::new(&self.state)
                                .bg(cx.theme().colors.secondary)
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
                                    // Fixed width container prevents layout shifting on model change
                                    div()
                                        .w_64() // adjust to rems(13.0) - rems(16.0) depending on typical model name length
                                        .flex_none()
                                        .child(
                                            Select::new(&self.models)
                                                .w_full()
                                                .bg(cx.theme().colors.secondary)
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
                                        .rounded_full()
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.generate(window, cx)
                                        })),
                                ),
                        ),
                ),
        )
    }
}
