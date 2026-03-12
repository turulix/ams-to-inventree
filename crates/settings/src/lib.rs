pub mod settings;
pub mod printer;

use crate::settings::Settings;
use std::sync::LazyLock;

pub static SETTINGS: LazyLock<Settings> =
    LazyLock::new(|| Settings::new().expect("Failed to Parse Config"));
