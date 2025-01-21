// Global dependencies
use crate::tool::format_time;
use gtk::glib::DateTime;
use std::fmt::{Display, Formatter, Result};

/// `Page` status
pub enum Status {
    Failure { time: DateTime },
    Input { time: DateTime },
    Loading { time: DateTime },
    New { time: DateTime },
    SessionRestore { time: DateTime },
    SessionRestored { time: DateTime },
    Success { time: DateTime },
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Failure { time } => {
                write!(f, "[{}] Failure", format_time(time))
            }
            Self::Input { time } => {
                write!(f, "[{}] Input issue", format_time(time))
            }
            Self::Loading { time } => {
                write!(f, "[{}] Loading...", format_time(time))
            }
            Self::New { time } => {
                write!(f, "[{}] New page", format_time(time))
            }
            Self::SessionRestore { time } => {
                write!(f, "[{}] Session restore...", format_time(time))
            }
            Self::SessionRestored { time } => {
                write!(f, "[{}] Session restored", format_time(time))
            }
            Self::Success { time } => {
                write!(f, "[{}] Success", format_time(time))
            }
        }
    }
}
