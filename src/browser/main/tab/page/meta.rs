use gtk::glib::GString;

// Page MIME type (not related with gemini status code)
// Useful for content renderer detection, etc
pub enum Mime {
    TextGemini,
    TextPlain,
}

// Internal page status (not related with gemini status code)
// Useful for widgets composition
pub enum Status {
    Failure,
    Redirect,
    Success,
}

pub struct Meta {
    // Text meta data for page
    // Useful to update window title, label text, etc
    pub title: Option<GString>,
    pub description: Option<GString>,
    // Enums
    pub mime: Option<Mime>,
    pub status: Option<Status>,
    // Useful to compose other widgets
    // (e.g. navigation bar listen for this value update)
    pub progress_fraction: f32,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            mime: None,
            status: None,
            progress_fraction: 0.0,
        }
    }
}
