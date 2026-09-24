use std::path::PathBuf;

use backend::project::Project;
use dioxus::prelude::*;
use firmware_targets::{TargetKind, TemplateKind};
use rfd::AsyncFileDialog;
use strum::IntoEnumIterator;

use crate::{
    components::{
        button::{Button, ButtonVariant},
        input::Input,
        select::{Select, SelectOption},
    },
    routes::Route,
};

static STYLE: Asset = asset!("/assets/new_project.css");

#[component]
pub fn NewProject() -> Element {
    let navigator = navigator();

    let mut name = use_signal(|| "MyProject".to_owned());
    let mut location = use_signal(String::new);
    let mut target = use_signal(|| TargetKind::iter().next());
    let mut template = use_signal(|| TemplateKind::iter().next());
    let mut error = use_signal(|| None::<String>);

    let browse = move |_| {
        spawn(async move {
            let Some(path) = AsyncFileDialog::new()
                .set_title("Choose project location")
                .pick_folder()
                .await
            else {
                return;
            };

            location.set(path.path().to_string_lossy().into_owned());
            error.set(None);
        });
    };

    let submit = move |_| {
        let project_name = name.read().trim().to_owned();
        if project_name.is_empty() {
            error.set(Some("Project name cannot be empty.".to_owned()));
            return;
        }

        let loc_str = location.read();
        if loc_str.trim().is_empty() {
            error.set(Some("Please select a project location.".to_owned()));
            return;
        }

        let root = PathBuf::from(loc_str.as_str()).join(&project_name);

        let Some(selected_target) = *target.read() else {
            error.set(Some("Target microcontroller missing.".to_owned()));
            return;
        };

        let Some(selected_template) = *template.read() else {
            error.set(Some("Template missing.".to_owned()));
            return;
        };

        let project = Project::new()
            .with_name(project_name)
            .with_root(root)
            .with_target(selected_target)
            .with_template(selected_template);

        let _ = project;
        navigator.push(Route::Home);
    };

    rsx! {
        document::Stylesheet { href: STYLE }

        div {
            class: "dialog-page",

            div {
                class: "dialog-container",

                // Header
                div {
                    class: "dialog-header",

                    h1 { class: "dialog-title", "New Project" }
                    p { class: "dialog-subtitle", "Create a new STM32 Studio project." }
                }

                // Form Stack
                div {
                    class: "dialog-form",

                    // 1. Project Name
                    div {
                        class: "form-group",

                        label { class: "form-label", "Project Name" }

                        div {
                            class: "input",
                            Input {
                                value: name,
                                oninput: move |event: FormEvent| {
                                    name.set(event.value());
                                    error.set(None);
                                },
                            }
                        }

                        // 2. Location
                        div {
                            class: "form-group",
                        }


                        label { class: "form-label", "Location" }

                        div {
                            class: "location-row",

                            div {
                                class: "input location-input",

                                Input {
                                    value: location,
                                    placeholder: "Choose project location...",
                                    oninput: move |event: FormEvent| {
                                        location.set(event.value());
                                        error.set(None);
                                    },
                                }
                            }

                            Button {
                                class: "gpui-btn-secondary",
                                variant: ButtonVariant::Secondary,
                                onclick: browse,
                                "Browse"
                            }
                        }
                    }

                    // 3. Firmware Configuration
                    div {
                        class: "form-group",

                        label {
                            class: "form-label",
                            "Firmware Configuration"
                        }

                        div {
                            class: "firmware-config-card",

                            // Target Group
                            div {
                                class: "config-col",

                                label {
                                    class: "config-col-label",
                                    "Firmware Target"
                                }

                                Select::<TargetKind> {
                                    class: "select select-left",
                                    value: Some(ReadSignal::new(target)),
                                    on_value_change: move |value: Option<TargetKind>| {
                                        target.set(value);
                                        error.set(None);
                                    },

                                    for (index, kind) in TargetKind::iter().enumerate() {
                                        SelectOption::<TargetKind> {
                                            index,
                                            value: kind,
                                            text_value: kind.to_string(),
                                            "{kind}"
                                        }
                                    }
                                }
                            }

                            // Template Group
                            div {
                                class: "config-col",

                                label {
                                    class: "config-col-label",
                                    "Template"
                                }

                                Select::<TemplateKind> {
                                    class: "select select-right",
                                    value: Some(ReadSignal::new(template)),
                                    on_value_change: move |value: Option<TemplateKind>| {
                                        template.set(value);
                                        error.set(None);
                                    },

                                    for (index, kind) in TemplateKind::iter().enumerate() {
                                        SelectOption::<TemplateKind> {
                                            index,
                                            value: kind,
                                            text_value: kind.to_string(),
                                            "{kind}"
                                        }
                                    }
                                }
                            }
                        }

                        p {
                            class: "form-hint",
                            "Choose the target microcontroller and firmware software stack."
                        }
                    }
                }

                // Error Section (Fixed height to prevent layout shift)
                div {
                    class: "error-container",

                    if let Some(message) = error.read().as_ref() {
                        div {
                            class: "error-badge",
                            "{message}"
                        }
                    }
                }

                // Actions
                div {
                    class: "dialog-actions",

                    Button {
                        class: "gpui-btn-secondary",
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| navigator.go_back(),
                        "Cancel"
                    }

                    Button {
                        class: "gpui-btn-primary",
                        variant: ButtonVariant::Primary,
                        onclick: submit,
                        "Create Project"
                    }
                }
            }
        }
    }
}
