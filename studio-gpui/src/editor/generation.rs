use backend::{Model, project::GenerationRequest};
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    base::{IndexPath, StyledExt, input::TextareaState},
    component::{
        ActiveTheme,
        button::{Button, ButtonVariants},
        input::Textarea,
        select::{SearchableVec, Select, SelectItem, SelectState},
    },
    div, rems,
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
                .max_w_128()
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
                    Select::new(&self.models)
                        .title_prefix("Model: ")
                        .menu_max_h(rems(10.0)),
                )
                .child(
                    div()
                        .relative()
                        .w_full()
                        .rounded_md()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().background)
                        .child(Textarea::new(&self.state).bordered(false).pb_12())
                        .child(
                            div().absolute().bottom_2().right_2().child(
                                Button::new("generate")
                                    .primary()
                                    .label("Generate")
                                    .on_click(
                                        cx.listener(|this, _, window, cx| {
                                            this.generate(window, cx)
                                        }),
                                    ),
                            ),
                        ),
                ),
        )
    }
}
