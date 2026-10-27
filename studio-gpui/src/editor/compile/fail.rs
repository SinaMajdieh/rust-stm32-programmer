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

pub struct BuildFailed;

impl BuildFailed {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for BuildFailed {
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
                        Icon::new(IconName::CircleX)
                            .size_12()
                            .text_color(theme.colors.danger),
                    )
                    .child(
                        v_flex()
                            .items_start()
                            .justify_center()
                            .child(
                                div()
                                    .text_3xl()
                                    .text_color(theme.colors.danger)
                                    .child("Build Failed"),
                            )
                            .child(
                                div()
                                    .text_color(theme.colors.muted_foreground)
                                    .text_center()
                                    .text_align(gpui_kit::TextAlign::Center)
                                    .child("There was an error while compiling your project"),
                            ),
                    ),
            )
            // Error message
            .child(
                Alert::error(
                    "banner-error",
                    "main.c:42:10: err: 'HAL_GPIO_TogglePin' undeclared (first use in this function)",
                )
                .text_lg()
                .banner()
                .icon(Icon::empty()),
            )
            // Recovery options
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
                            // Fix with AI
                            .child(
                                Button::new("fix-with-ai")
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
                                            .child(Icon::new(IconName::Sparkles).size_8())
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
                                                            .child("Fix with AI"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .text_color(theme.colors.muted)
                                                            .child(
                                                                "Send the error and context to the model to fix the issue",
                                                            ),
                                                    ),
                                            ),
                                    ),
                            )
                            // Rebuild
                            .child(
                                Button::new("rebuild")
                                    .w_full()
                                    .min_h_24()
                                    .px_6()
                                    .py_4()
                                    .rounded_xl()
                                    .border_2()
                                    .border_color(theme.colors.border)
                                    .bg(theme.colors.background)
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .items_center()
                                            .justify_start()
                                            .gap_4()
                                            .child(Icon::new(IconName::Hammer).size_8())
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
                                                            .child("Rebuild"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .text_color(theme.colors.muted)
                                                            .child(
                                                                "Try compiling again with the current files",
                                                            ),
                                                    ),
                                            ),
                                    ),
                            )
                            // Edit code
                            .child(
                                Button::new("edit-code")
                                    .w_full()
                                    .min_h_24()
                                    .px_6()
                                    .py_4()
                                    .rounded_xl()
                                    .border_2()
                                    .border_color(theme.colors.border)
                                    .bg(theme.colors.background)
                                    .child(
                                        h_flex()
                                            .w_full()
                                            .items_center()
                                            .justify_start()
                                            .gap_4()
                                            .child(Icon::new(IconName::SquarePen).size_8())
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
                                                            .child("Edit Code"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .text_color(theme.colors.muted)
                                                            .child(
                                                                "Open the files in the explorer",
                                                            ),
                                                    ),
                                            ),
                                    ),
                            ),
                    ),
            )
    }
}
