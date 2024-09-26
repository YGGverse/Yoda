use gtk::glib::{markup_escape_text, GString};

pub struct Plain {
    markup: GString,
    source: GString,
}

impl Plain {
    pub fn from(line: &str) -> Plain {
        Self {
            markup: GString::from(format!("{}\n", markup_escape_text(line))),
            source: GString::from(line),
        }
    }

    pub fn markup(&self) -> &GString {
        &self.markup
    }

    pub fn source(&self) -> &GString {
        &self.source
    }
}
