use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::{StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, Icon,
        alert::Alert,
        button::{Button, ButtonVariants},
    },
    div,
};

pub struct GenerationFailed;

impl GenerationFailed {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for GenerationFailed {
    fn render(
        &mut self,
        _window: &mut gpui_kit::Window,
        cx: &mut gpui_kit::prelude::Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        div()
            .size_full()
            .v_flex()
            .justify_start()
            .border_1()
            .border_color(theme.colors.border)
            .rounded_md()
            .px_8()
            .py_12()
            .gap_8()
            // Header
            .child(
                h_flex()
                    .w_full()
                    .items_center()
                    .justify_start()
                    .gap_4()
                    .child(
                        div().child(
                            Icon::new(IconName::CircleX)
                                .size_12()
                                .text_color(theme.colors.danger),
                        ),
                    )
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(
                                div()
                                    .text_color(theme.colors.danger)
                                    .text_3xl()
                                    .child("Generation Failed"),
                            )
                            .child(
                                div()
                                    .text_color(theme.colors.muted_foreground)
                                    .text_center()
                                    .text_align(gpui_kit::TextAlign::Center)
                                    .child("There was an error while generating code"),
                            ),
                    ),
            )
            .child(
                Alert::error("banner-error", "Generation was timedout.")
                    .text_lg()
                    .banner()
                    .icon(Icon::empty()),
            )
            .child(
                v_flex()
                    .items_stretch()
                    .justify_start()
                    .gap_3()
                    .child(div().text_3xl().child("What would you like to do?"))
                    .child(
                        v_flex()
                            .w_full()
                            .gap_3()
                            .child(
                                Button::new("regenerate")
                                    .primary()
                                    .w_full()
                                    .min_h_24()
                                    .px_6()
                                    .py_4()
                                    .rounded_xl()
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .items_center()
                                            .justify_start()
                                            .gap_4()
                                            .child(Icon::new(IconName::RotateCcw).size_8())
                                            .child(
                                                v_flex()
                                                    .w_full()
                                                    .items_start()
                                                    .justify_center()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_2xl()
                                                            .font_bold()
                                                            .child("Regenerate"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .text_color(theme.colors.muted)
                                                            .child("Try generating again with the current prompt"),
                                                    ),
                                            )
                                    ),
                            )
                            .child(
                                Button::new("back-to-prompt")
                                    .w_full()
                                    .min_h_24()
                                    .px_6()
                                    .py_4()
                                    .rounded_xl()
                                    .border_2()
                                    .border_color(theme.colors.border)
                                    .bg(theme.colors.secondary)
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .items_center()
                                            .justify_start()
                                            .gap_4()
                                            .child(Icon::new(IconName::SquareSlash).size_8())
                                            .child(
                                                v_flex()
                                                    .w_full()
                                                    .items_start()
                                                    .justify_center()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_2xl()
                                                            .font_bold()
                                                            .child("Back to prompt"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .text_color(theme.colors.muted_foreground)
                                                            .child("Try chaging the propt or switching to a different model."),
                                                    ),
                                            )
                                    ),
                            ),
                    )
            )
    }
}
