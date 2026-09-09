use std::{rc::Rc, str::FromStr};

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use strum::IntoEnumIterator;

use firmware_targets::{TargetKind, TemplateKind};

slint::include_modules!();

fn enum_model<T>() -> ModelRc<SharedString>
where
    T: IntoEnumIterator + ToString,
{
    let values = T::iter()
        .map(|value| SharedString::from(value.to_string()))
        .collect::<Vec<_>>();

    ModelRc::from(Rc::new(VecModel::from(values)))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    // ─────────────────────────────────────────────────────────────
    // ComboBox models
    // ─────────────────────────────────────────────────────────────

    ui.set_targets(enum_model::<TargetKind>());
    ui.set_templates(enum_model::<TemplateKind>());

    // Add your model list here.
    //
    // For example:
    //
    // let models = vec![
    //     SharedString::from("qwen3.5:9b"),
    //     SharedString::from("qwen2.5-coder:7b"),
    // ];
    //
    // ui.set_models(ModelRc::from(Rc::new(VecModel::from(models))));

    let models = vec![
        SharedString::from("qwen3.5:9b"),
        SharedString::from("qwen2.5-coder:7b"),
    ];

    ui.set_models(ModelRc::from(Rc::new(VecModel::from(models))));

    // ─────────────────────────────────────────────────────────────
    // Target selection
    // ─────────────────────────────────────────────────────────────

    ui.on_target_changed(|value| {
        let target =
            TargetKind::from_str(value.as_str()).expect("Slint returned an invalid target");

        println!("Target selected: {target:?}");
    });

    // ─────────────────────────────────────────────────────────────
    // Template selection
    // ─────────────────────────────────────────────────────────────

    ui.on_template_changed(|value| {
        let template =
            TemplateKind::from_str(value.as_str()).expect("Slint returned an invalid template");

        println!("Template selected: {template:?}");
    });

    // ─────────────────────────────────────────────────────────────
    // Model selection
    // ─────────────────────────────────────────────────────────────

    ui.on_model_changed(|value| {
        println!("Model selected: {value}");
    });

    // ─────────────────────────────────────────────────────────────
    // Start UI
    // ─────────────────────────────────────────────────────────────

    ui.run()?;

    Ok(())
}
