use slint::{Model, ModelRc, SharedString, VecModel};

use crate::MainWindow;

pub fn initialize(window: &MainWindow) {
    window.set_log_entries(ModelRc::new(
        VecModel::<SharedString>::default(),
    ));
}

pub fn reset(window: &MainWindow) {
    clear_log(window);
    window.set_progress_value(0.0);
}

pub fn begin_programming(window: &MainWindow) {
    reset(window);
    window.set_is_programming(true);
    window.set_follow_log(true);
}

pub fn append_log(
    window: &MainWindow,
    message: impl Into<SharedString>,
) {
    with_log_model(window, |log| log.push(message.into()));
}

fn clear_log(window: &MainWindow) {
    with_log_model(window, VecModel::clear);
}

fn with_log_model(
    window: &MainWindow,
    action: impl FnOnce(&VecModel<SharedString>),
) {
    let model = window.get_log_entries();
    let log = model
        .as_any()
        .downcast_ref::<VecModel<SharedString>>()
        .expect("log_entries must use a VecModel");

    action(log);
}
