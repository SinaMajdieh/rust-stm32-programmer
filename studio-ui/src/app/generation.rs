// use std::rc::Rc;

// use firmware_targets::{TargetKind, TemplateKind};
// use slint::{ModelRc, SharedString, VecModel};
// use strum::IntoEnumIterator;

// slint::include_modules!();

// fn enum_model<T>() -> ModelRc<SharedString>
// where
//     T: IntoEnumIterator + ToString,
// {
//     let values = T::iter()
//         .map(|value| SharedString::from(value.to_string()))
//         .collect::<Vec<_>>();

//     ModelRc::from(Rc::new(VecModel::from(values)))
// }

// fn generate(ui: &AppWindow) {
//     // ─────────────────────────────────────────────────────────────
//     // ComboBox models
//     // ─────────────────────────────────────────────────────────────

//     ui.set_targets(enum_model::<TargetKind>());
//     ui.set_templates(enum_model::<TemplateKind>());

//     // Add your model list here.
//     //
//     // For example:
//     //
//     // let models = vec![
//     //     SharedString::from("qwen3.5:9b"),
//     //     SharedString::from("qwen2.5-coder:7b"),
//     // ];
//     //
//     // ui.set_models(ModelRc::from(Rc::new(VecModel::from(models))));

//     let models = vec![
//         SharedString::from("qwen3.5:9b"),
//         SharedString::from("qwen2.5-coder:7b"),
//     ];

//     ui.set_models(ModelRc::from(Rc::new(VecModel::from(models))));

//     ui.on_run(|target, template, model, prompt| {
//         println!("target: {target}");
//         println!("template: {template}");
//         println!("model: {model}");
//         println!("prompt: {prompt}");
//     });
// }
