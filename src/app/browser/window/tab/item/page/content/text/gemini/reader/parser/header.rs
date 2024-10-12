use gtk::glib::{Regex, RegexCompileFlags, RegexMatchFlags};

pub enum Level {
    H1,
    H2,
    H3,
}

pub struct Header {
    value: String,
    level: Level,
}

impl Header {
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let parsed = Regex::split_simple(
            r"^(#{1,3})\s*(.+)$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect header level
        let level = parsed.get(1)?;

        let level = match level.len() {
            1 => Level::H1,
            2 => Level::H2,
            3 => Level::H3,
            _ => return None,
        };

        // Detect header value
        let value = parsed.get(2)?;

        if value.trim().is_empty() {
            return None;
        }

        // Result
        Some(Header {
            level,
            value: String::from(value.as_str()),
        })
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}
