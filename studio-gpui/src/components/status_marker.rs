use gpui_kit::{
    App, IntoElement, ParentElement, Styled,
    assets::IconName,
    component::{
        ActiveTheme, Icon, Sizable,
        marker::{Marker, MarkerContent, MarkerIcon},
        spinner::Spinner,
    },
};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum StageStatus {
    Waiting,
    Running,
    Succeeded,
    Failed,
}

pub fn stage_marker(label: &'static str, status: StageStatus, cx: &App) -> impl IntoElement {
    let theme = cx.theme();

    match status {
        StageStatus::Waiting => Marker::new()
            .text_xl()
            .icon(
                MarkerIcon::new()
                    .child(Icon::new(IconName::Circle).large())
                    .text_color(theme.colors.muted_foreground),
            )
            .content(
                MarkerContent::new()
                    .text_color(theme.colors.foreground)
                    .ml_2()
                    .text(label),
            ),

        StageStatus::Running => Marker::new()
            .text_xl()
            .icon(
                MarkerIcon::new().child(
                    Spinner::new()
                        .icon(IconName::LoaderCircle)
                        .color(theme.colors.info)
                        .large(),
                ),
            )
            .content(
                MarkerContent::new()
                    .text_color(theme.colors.foreground)
                    .ml_2()
                    .text(label),
            ),

        StageStatus::Succeeded => Marker::new()
            .text_xl()
            .icon(
                MarkerIcon::new()
                    .child(Icon::new(IconName::CircleCheck).large())
                    .text_color(theme.colors.success),
            )
            .content(
                MarkerContent::new()
                    .text_color(theme.colors.foreground)
                    .ml_2()
                    .text(label),
            ),

        StageStatus::Failed => Marker::new()
            .text_xl()
            .icon(
                MarkerIcon::new()
                    .child(Icon::new(IconName::CircleX).large())
                    .text_color(theme.colors.danger),
            )
            .content(
                MarkerContent::new()
                    .text_color(theme.colors.foreground)
                    .ml_2()
                    .text(label),
            ),
    }
}
