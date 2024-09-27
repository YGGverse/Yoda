use gtk::glib::GString;

pub enum Mime {
    TextGemini,
    TextPlain,
}

pub struct Meta {
    pub title: Option<GString>,
    pub description: Option<GString>,
    pub mime: Option<Mime>,
    pub progress_fraction: f32,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            mime: None,
            progress_fraction: 0.0,
        }
    }
}
