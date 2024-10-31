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
    pub fn new_failure(
        title: Option<&str>,
        description: Option<&str>,
        icon_name: Option<&str>,
    ) -> Self {
        Self {
            gobject: failure::new_gobject_from(title, description, icon_name),
        }
    }

    /// Create new loading preset
    ///
    /// Useful as placeholder widget for async operations
    pub fn new_loading(
        title: Option<&str>,
        description: Option<&str>,
        show_with_delay: Option<Duration>,
    ) -> Self {
        Self {
            gobject: loading::new_gobject_from(title, description, show_with_delay),
        }
    }

    // Setters

    /// Set new description for status component
    ///
    /// Useful for loading widgets to update byte totals and other dynamically changed information
    ///
    /// Return `Self` reference to apply another functions in chain
    pub fn set_description(&self, description: Option<&str>) -> &Self {
        self.gobject.set_description(description);
        &self
    }

    // Getters

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
