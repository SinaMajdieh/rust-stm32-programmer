use backend::Model;
use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    base::{IndexPath, StyledExt, input::TextareaState},
    component::{
        ActiveTheme,
        input::Textarea,
        select::{SearchableVec, Select, SelectItem, SelectState},
    },
    div, rems,
};

use crate::settings::Settings;

pub struct Generation {
    state: Entity<TextareaState>,
    models: Entity<SelectState<SearchableVec<ModelOption>>>,
}

#[derive(Clone)]
struct ModelOption(Model);

impl SelectItem for ModelOption {
    type Value = Model;
    fn title(&self) -> gpui_kit::SharedString {
        self.0.name().into()
    }
    fn value(&self) -> &Self::Value {
        &self.0
    }
}

impl Generation {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings = cx.try_global::<Settings>().unwrap();
        let models = settings
            .config
            .llm
            .generator
            .available_models
            .iter()
            .cloned()
            .map(ModelOption)
            .collect::<Vec<_>>();

        let models = SearchableVec::from(models);

        let models = cx.new(|cx| {
            SelectState::new(models, Some(IndexPath::default()), window, cx).searchable(true)
        });

        let state = cx.new(|cx| {
            TextareaState::new(window, cx)
                .auto_grow(2, 8)
                .placeholder("What would you like to build?")
        });
        Self { state, models }
    }
}

impl Render for Generation {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .v_flex()
            .items_center()
            .justify_center()
            .p_8()
            .child(
                div()
                    .w_full()
                    .max_w_1_2()
                    .v_flex()
                    .gap_4()
                    .child(
                        div()
                            .font_semibold()
                            .text_xl()
                            .text_color(cx.theme().foreground)
                            .child("What would you like to build?"),
                    )
                    .child(Textarea::new(&self.state).h_24().bordered(true))
                    .child(
                        Select::new(&self.models)
                            .title_prefix("Model: ")
                            .menu_max_h(rems(10.0)),
                    ),
            )
    }
}
