use gtk::glib::GString;

pub enum Status {
    // SensitiveInput,
    // Complete,
    Failure,
    Input,
    Connecting,
    Connected,
    // ProxyNegotiated,
    // ProxyNegotiating,
    Redirect,
    Reload,
    // Request,
    // Resolved,
    // Resolving,
    // Response,
    Success,
    // TlsHandshaked,
    // TlsHandshaking,
} // @TODO

pub struct Meta {
    pub title: Option<GString>,
    pub description: Option<GString>,
    pub status: Option<Status>,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            status: None,
        }
    }
}
