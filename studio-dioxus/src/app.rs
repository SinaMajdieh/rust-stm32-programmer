use crate::routes::Route;
use dioxus::prelude::*;

static THEME: Asset = asset!("/assets/theme.css");
static SPACING: Asset = asset!("/assets/spacing.css");
static COMPONENTS: Asset = asset!("/assets/dx-components-theme.css");
static APP_STYLE: Asset = asset!("/assets/app.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: THEME }
        document::Stylesheet { href: SPACING }
        document::Stylesheet { href: COMPONENTS }
        document::Stylesheet { href: APP_STYLE }

        div {
            class: "desktop-app-window",
            Router::<Route> {}
        }
    }
}
