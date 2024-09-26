use gtk::glib::{gformat, markup_escape_text, GString};

pub struct Plain {
    markup: GString,
}

impl Plain {
    pub fn from(line: &str) -> Plain {
        Self {
            markup: gformat!("{}\n", markup_escape_text(line)),
        }
    }

    pub fn markup(&self) -> &GString {
        &self.markup
    }
}
