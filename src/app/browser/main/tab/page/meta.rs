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
    Connect,
    Failure,
    Prepare,
    Redirect,
    Reload,
    Request,
    Response,
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
}

impl Meta {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            mime: None,
            status: None,
        }
    }
}
