use dioxus::prelude::*;
use dioxus_icons::lucide::{FolderOpen, Plus};
use rfd::AsyncFileDialog;

use crate::routes::Route;

static STYLE: Asset = asset!("/assets/home.css");

#[component]
pub fn Home() -> Element {
    let navigator = navigator();

    let open_project = move |_| {
        spawn(async move {
            let Some(folder) = AsyncFileDialog::new()
                .set_title("Open STM32 Studio Project")
                .pick_folder()
                .await
            else {
                return;
            };

            let _path = folder.path();
            // TODO: Load project and navigate
        });
    };

    rsx! {
        document::Stylesheet { href: STYLE }

        div {
            class: "home-view",

            div {
                class: "home-container",

                // Header
                div {
                    class: "home-header",

                    h1 {
                        class: "home-title",
                        "Welcome back"
                    }

                    p {
                        class: "home-subtitle",
                        "What would you like to do?"
                    }
                }

                // Action Cards
                div {
                    class: "home-actions",

                    div {
                        class: "action-card",
                        onclick: move |_| {
                            navigator.push(Route::NewProject);
                        },

                        div {
                            class: "card-icon",
                            Plus { size: 28 }
                        }

                        div {
                            class: "card-body",
                            div { class: "card-title", "New Project" }
                            div { class: "card-description", "Create a new project" }
                        }
                    }

                    div {
                        class: "action-card",
                        onclick: open_project,

                        div {
                            class: "card-icon",
                            FolderOpen { size: 28 }
                        }

                        div {
                            class: "card-body",
                            div { class: "card-title", "Open Project" }
                            div { class: "card-description", "Open an existing project" }
                        }
                    }
                }
            }
        }
    }
}
