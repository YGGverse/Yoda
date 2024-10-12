use gtk::glib::{gformat, GString};

pub struct Plain {
    line: GString,
}

impl Plain {
    // Construct
    pub fn from(line: &str) -> Self {
        Self {
            line: gformat!("{}\n", line),
        }
    }

    // Getters
    pub fn as_str(&self) -> &str {
        self.line.as_str()
    }
}
