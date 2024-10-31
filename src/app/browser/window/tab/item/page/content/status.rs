mod failure;
mod loading;

use adw::StatusPage;
use std::time::Duration;

pub struct Status {
    gobject: StatusPage,
}

impl Status {
    // Constructors

    /// Create new failure preset
    ///
    /// Useful as placeholder widget for error handlers
    pub fn new_failure() -> Self {
        Self {
            gobject: failure::new_gobject(),
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
        &self
    }

    /// Set new description for `Self`
    ///
    /// Useful for loading widgets to update byte totals and other dynamically changed information
    ///
    /// Return `Self` reference to apply another functions in chain
    pub fn set_description(&self, value: Option<&str>) -> &Self {
        self.gobject.set_description(value);
        &self
    }

    // Getters

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
