use std::fmt::Display;
use std::str::FromStr;
use chrono::Utc;

#[derive(Clone)]
pub enum Time {
    Rfc2822,
    Rfc3339,
    Custom(String)
}

impl Default for Time {
    fn default() -> Self {
        Time::Rfc2822
    }
}

impl From<String> for Time {
    fn from(s: String) -> Self {
        if s == "rfc2822" {
            Time::Rfc2822
        }
        else if s == "rfc3339" {
            Time::Rfc3339
        } else {
            Time::Custom(s)
        }
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rfc2822" => Ok(Time::Rfc2822),
            "rfc3339" => Ok(Time::Rfc3339),
            _ => Ok(Time::Custom(s.to_string())),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Time::Rfc2822 => write!(f, "{}", Utc::now().to_rfc2822()),
            Time::Rfc3339 => write!(f, "{}", Utc::now().to_rfc3339()),
            Time::Custom(ref s) => write!(f, "{}", Utc::now().format(s)),
        }
    }
}