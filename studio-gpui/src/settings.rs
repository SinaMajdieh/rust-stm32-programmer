use std::path::Path;

use backend::{Config, ConfigError};
use gpui_kit::{App, Global};

pub struct Settings {
    pub config: Config,
}

impl Settings {
    pub fn load(path: impl AsRef<Path>, cx: &mut App) -> Result<(), ConfigError> {
        let config = Config::load(path)?;
        let settings = Self { config };
        cx.set_global(settings);
        Ok(())
    }
}

impl Global for Settings {}
