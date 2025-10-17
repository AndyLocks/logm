use std::fmt::Display;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Clone)]
pub enum Levels {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl FromStr for Levels {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "warn" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "err" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid log level")),
        }
    }
}

impl Default for Levels {
    fn default() -> Self {
        Self::Info
    }
}

impl Display for Levels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Debug => write!(f, "DEBUG"),
            Self::Info => write!(f, "INFO"),
            Self::Warning => write!(f, "WARN"),
            Self::Error => write!(f, "ERROR"),
            Self::Fatal => write!(f, "FATAL"),
        }
    }
}