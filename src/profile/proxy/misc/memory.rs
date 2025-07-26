mod bool;
use bool::Bool;

// Shared values

const HIGHLIGHT_REQUEST_ENTRY: &str = "highlight_request_entry";

pub enum Memory {
    HighlightRequestEntry(Bool),
}

impl Memory {
    // Constructors

    pub fn from_db_row(key: &str, value: Option<String>) -> Option<Self> {
        if key == HIGHLIGHT_REQUEST_ENTRY {
            Some(Self::HighlightRequestEntry(Bool::from_db_value(value)))
        } else {
            None
        }
    }

    pub fn highlight_request_entry(value: bool) -> Self {
        Memory::HighlightRequestEntry(Bool::from(value))
    }

    // Actions

    pub fn into_db_row(self) -> (String, String) {
        match self {
            Self::HighlightRequestEntry(value) => {
                (HIGHLIGHT_REQUEST_ENTRY.to_string(), value.into_db_value())
            }
        }
    }

    // Getters

    pub fn key(&self) -> &str {
        match self {
            Self::HighlightRequestEntry(..) => HIGHLIGHT_REQUEST_ENTRY,
        }
    }
}
