use gtk::glib::{gformat, GString};

pub struct Plain {
    // nothing yet..
}

impl Plain {
    pub fn from(line: &str) -> GString {
        gformat!("{}\n", line)
    }
}
