use gtk::glib::{GString, Regex, RegexCompileFlags, RegexMatchFlags};

pub struct List {
    pub value: GString,
}

impl List {
    pub fn from(line: &str) -> Option<Self> {
        // Parse line
        let regex = Regex::split_simple(
            r"^\*\s*(.+)$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect value
        let value = regex.get(1)?;

        if value.trim().is_empty() {
            return None;
        }

        // Result
        Some(Self {
            value: GString::from(value.as_str()),
        })
    }
}
