pub mod download;
mod failure;
mod identity;
mod loading;
mod mime;

use adw::StatusPage;
use gtk::gio::{Cancellable, File};
use std::{rc::Rc, time::Duration};

pub struct Status {
    gobject: StatusPage,
}

impl Status {
    // Constructors

    /// Create new download preset
    pub fn new_download(
        initial_filename: &str,
        cancellable: &Cancellable,
        on_choose: impl Fn(File, Rc<download::Action>) + 'static,
    ) -> Self {
        Self {
            gobject: download::new(initial_filename, cancellable, on_choose),
        }
    }

    /// Create new failure preset
    ///
    /// Useful as placeholder widget for error handlers
    pub fn new_failure() -> Self {
        Self {
            gobject: failure::new_gobject(),
        }
    }

    /// Create new mime issue preset
    ///
    /// Useful as placeholder widget for mime issue handlers
    pub fn new_mime() -> Self {
        Self {
            gobject: mime::new_gobject(),
        }
    }

    /// Create new identity preset
    ///
    /// Useful as placeholder for 60 status code
    /// https://geminiprotocol.net/docs/protocol-specification.gmi#status-60
    pub fn new_identity(action: Rc<crate::app::browser::window::tab::item::Action>) -> Self {
        Self {
            gobject: identity::new_gobject(action),
        }
    }

    /// Create new loading preset
    ///
    /// Useful as placeholder widget for async operations
    pub fn new_loading(show_with_delay: Option<Duration>) -> Self {
        Self {
            gobject: loading::new_gobject(show_with_delay),
        }
    }

    // Setters

    /// Set new title for `Self`
    ///
    /// Return `Self` reference to apply another functions in chain
    pub fn set_title(&self, value: &str) -> &Self {
        self.gobject.set_title(value);
        self
    }

    /// Set new description for `Self`
    ///
    /// Useful for loading widgets to update byte totals and other dynamically changed information
    ///
    /// Return `Self` reference to apply another functions in chain
    pub fn set_description(&self, value: Option<&str>) -> &Self {
        self.gobject.set_description(value);
        self
    }

    // Getters

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
