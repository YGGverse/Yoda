mod failure;
mod loading;

use failure::Failure;
use loading::Loading;

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
    pub fn new_failure(title: Option<&str>, description: Option<&str>) -> Self {
        Self {
            gobject: Failure::new(title, description).gobject().clone(),
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
            gobject: Loading::new(title, description, show_with_delay)
                .gobject()
                .clone(),
        }
    }

    // Setters

    /// Set new description for status component
    ///
    /// Useful for loading widgets to update byte totals and other dynamically changed information
    pub fn set_description(&self, description: Option<&str>) {
        self.gobject.set_description(description);
    }

    // Getters

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
