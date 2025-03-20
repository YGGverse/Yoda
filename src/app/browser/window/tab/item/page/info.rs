// Public dependencies

pub mod event;
pub use event::Event;

// Local dependencies

use gtk::gio::NetworkAddress;

/// Common, shared `Page` information holder
/// * used for the Information dialog window on request indicator activate
/// * collecting by the page driver implementation, using public API
pub struct Info {
    /// Hold page events like connection phase and parsing time
    event: Vec<Event>,
    /// Page content type
    mime: Option<String>,
    /// Hold redirections chain with handled details
    /// * the `referrer` member name is reserved for other protocols
    redirect: Option<Box<Self>>,
    /// Optional remote host details
    /// * useful also for geo-location feature
    remote: Option<NetworkAddress>,
    /// Key to relate data collected with the specific request
    request: Option<String>,
    /// Hold page content size
    size: Option<usize>,
}

impl Info {
    // Constructors

    /// Create new empty `Self` with expected default capacity
    pub fn new() -> Self {
        Self {
            event: Vec::with_capacity(50), // estimated max events quantity for all drivers
            mime: None,
            redirect: None,
            remote: None,
            request: None,
            size: None,
        }
    }

    // Setters
    // useful to update `Self` as chain of values

    /// Take `Self`, convert it into the redirect member,
    /// then, return new `Self` back
    /// * tip: use on driver redirection events
    pub fn into_redirect(self) -> Self {
        let mut this = Self::new();
        this.redirect = Some(Box::new(self));
        this
    }

    pub fn add_event(&mut self, name: String) -> &mut Self {
        self.event.push(Event::now(name));
        self
    }

    pub fn set_mime(&mut self, mime: Option<String>) -> &mut Self {
        self.mime = mime;
        self
    }

    pub fn set_remote(&mut self, remote: Option<NetworkAddress>) -> &mut Self {
        self.remote = remote;
        self
    }

    pub fn set_request(&mut self, request: Option<String>) -> &mut Self {
        self.request = request;
        self
    }

    pub fn set_size(&mut self, size: Option<usize>) -> &mut Self {
        self.size = size;
        self
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}
