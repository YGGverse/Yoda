use gtk::glib::{GString, Regex, RegexCompileFlags, RegexMatchFlags};

pub enum Level {
    H1,
    H2,
    H3,
}

pub struct Header {
    pub value: GString,
    pub level: Level,
}

impl Header {
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let regex = Regex::split_simple(
            r"^(#{1,3})\s*(.+)$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect header level
        let level = regex.get(1)?;

        let level = match level.len() {
            1 => Level::H1,
            2 => Level::H2,
            3 => Level::H3,
            _ => return None,
        };

        // Detect header value
        let value = regex.get(2)?;

        if value.trim().is_empty() {
            return None;
        }

        // Result
        Some(Header {
            level,
            value: GString::from(value.as_str()),
        })
    }
}
