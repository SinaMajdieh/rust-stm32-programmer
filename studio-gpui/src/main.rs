mod home;
pub mod project;
mod studio;
mod workspace;

use std::path::PathBuf;

use gpui_kit::{
    component::{Root, Theme, ThemeRegistry},
    *,
};

use crate::studio::Studio;

fn main() {
    gpui_kit::application().run(|cx: &mut App| {
        gpui_kit::init(cx);
        init_theme(cx);

        let bounds = Bounds::centered(None, size(px(800.0), px(582.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("STM32 Studio".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                window_min_size: Some(size(px(640.0), px(540.0))),
                ..Default::default()
            },
            |window, cx| {
                let studio = cx.new(|cx| Studio::new(window, cx));
                cx.new(|cx| Root::new(studio, window, cx))
            },
        )
        .expect("failed to open STM32 Studio");

        cx.activate(true);
    });
}

pub fn init_theme(cx: &mut App) {
    let theme_name = SharedString::from("STM32 Studio Dark");

    let themes_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("themes");

    ThemeRegistry::watch_dir(themes_path, cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    })
    .expect("failed to load GPUI themes");
}
