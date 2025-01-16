use gtk::glib::{GString, Uri};

pub enum Input {
    Response { base: Uri, title: GString },
    Sensitive { base: Uri, title: GString },
    Titan { base: Uri },
}
