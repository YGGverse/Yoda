use gtk::glib::GString;

pub enum Status {
    Complete,
    Failure,
    Input,
    Connecting,
    Connected,
    ProxyNegotiated,
    ProxyNegotiating,
    Redirect,
    Reload,
    Resolved,
    Resolving,
    Success,
    TlsHandshaked,
    TlsHandshaking,
}

pub struct Meta {
    pub title: Option<GString>,
    //pub description: Option<GString>,
    pub status: Option<Status>,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            title: None,
            //description: None,
            status: None,
        }
    }
}
