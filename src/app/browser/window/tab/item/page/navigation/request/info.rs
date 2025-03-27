mod dialog;
mod event;
mod redirect;
mod socket;

use super::Profile;
use dialog::Dialog;
use event::Event;
use gtk::{gio::SocketAddress, prelude::IsA};
use redirect::Redirect;
use socket::Socket;

/// Common, shared `Page` information holder
/// * used for the Information dialog window on request indicator activate
/// * collecting by the page driver implementation, using public API
pub struct Info {
    /// Hold page events like connection phase and parsing time
    event: Vec<Event>,
    /// Hold optional header string to dump it in the info dialog
    /// and calculate total size
    header: Option<String>,
    /// Page content type
    mime: Option<String>,
    /// Hold redirections chain with handled details
    /// * the `referrer` member name is reserved for other protocols
    redirect: Option<Redirect>,
    /// Key to relate data collected with the specific request
    request: Option<String>,
    /// Hold size info
    size: Option<usize>,
    /// Optional socket details
    /// * useful also for geo-location feature
    socket: Option<Socket>,
}

impl Info {
    // Constructors

    /// Create new empty `Self` with expected default capacity
    pub fn new() -> Self {
        Self {
            event: Vec::with_capacity(50), // estimated max events quantity for all drivers
            header: None,
            mime: None,
            redirect: None,
            request: None,
            size: None,
            socket: None,
        }
    }

    pub fn into_permanent_redirect(self) -> Self {
        let mut this = Self::new();
        this.redirect = Some(Redirect::permanent(self));
        this
    }

    pub fn into_temporary_redirect(self) -> Self {
        let mut this = Self::new();
        this.redirect = Some(Redirect::temporary(self));
        this
    }

    // Actions

    pub fn dialog(&self, parent: &impl IsA<gtk::Widget>, profile: &Profile) {
        use adw::{PreferencesDialog, prelude::AdwDialogExt};
        PreferencesDialog::info(profile, self).present(Some(parent))
    }

    // Setters
    // * update `Self` in chain

    /// Reset `Self` to the clean state
    /// * this method keeps `Redirect` value!
    pub fn reset(&mut self, is_unset_redirect: bool) -> &mut Self {
        if is_unset_redirect {
            self.set_redirect(None);
        }
        self.clear_events()
            .set_header(None)
            .set_mime(None)
            .set_request(None)
            .set_size(None)
            .set_socket(None)
    }

    pub fn add_event(&mut self, name: String) -> &mut Self {
        self.event.push(Event::now(name));
        self
    }

    pub fn clear_events(&mut self) -> &mut Self {
        self.event.clear();
        self
    }

    pub fn set_header(&mut self, header: Option<String>) -> &mut Self {
        self.header = header;
        self
    }

    pub fn set_mime(&mut self, mime: Option<String>) -> &mut Self {
        self.mime = mime;
        self
    }

    pub fn set_redirect(&mut self, redirect: Option<Redirect>) -> &mut Self {
        self.redirect = redirect;
        self
    }

    pub fn set_socket(&mut self, address: Option<(SocketAddress, SocketAddress)>) -> &mut Self {
        self.socket = address.map(|(local_address, remote_address)| Socket {
            local_address,
            remote_address,
        });
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

    // Getters

    pub fn request(&self) -> Option<&str> {
        self.request.as_deref()
    }

    pub fn matches(&self, request: &str) -> bool {
        self.request.as_ref().is_some_and(|r| r == request)
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}
