use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    base::{StyledExt, input::TextareaState},
    component::{ActiveTheme, input::Textarea},
    div,
};

pub struct Generation {
    state: Entity<TextareaState>,
}

impl Generation {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| {
            TextareaState::new(window, cx)
                .auto_grow(2, 8)
                .placeholder("Generate Code")
        });
        Self { state }
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
            .gap_4()
            .child(
                div()
                    .font_semibold()
                    .text_xl()
                    .text_color(cx.theme().foreground)
                    .child("What would you like to build?"),
            )
            .child(Textarea::new(&self.state).h_24().bordered(true).max_w_1_2())
    }
}
