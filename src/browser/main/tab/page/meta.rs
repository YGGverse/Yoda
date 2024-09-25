use gtk::glib::GString;

pub enum Mime {
    Undefined,
    TextGemini,
    TextPlain,
}

pub struct Meta {
    pub title: GString,
    pub description: GString,
    pub mime: Mime,
    pub progress_fraction: f32,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            title: GString::new(),
            description: GString::new(),
            mime: Mime::Undefined,
            progress_fraction: 0.0,
        }
    }
}
