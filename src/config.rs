use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub debug: String,
    pub info: String,
    pub warning: String,
    pub error: String,
    pub fatal: String,
    pub message: String,
    pub time: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: "white".into(),
            info: "green".into(),
            warning: "yellow".into(),
            error: "red".into(),
            fatal: "red".into(),
            message: "white".into(),
            time: "white".into(),
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| confy::load("logm", "logm").unwrap_or_default());