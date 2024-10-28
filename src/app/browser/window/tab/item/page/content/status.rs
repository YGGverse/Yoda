mod failure;
mod loading;

use failure::Failure;
use loading::Loading;

use adw::StatusPage;

pub struct Status {
    gobject: StatusPage,
}

impl Status {
    // Constructors

    /// Create new default failure component
    pub fn new_failure(title: Option<&str>, description: Option<&str>) -> Self {
        Self {
            gobject: Failure::new(title, description).gobject().clone(),
        }
    }

    /// Create new default loading component
    ///
    /// Useful as the placeholder widget for async operations
    pub fn new_loading(title: Option<&str>, description: Option<&str>) -> Self {
        Self {
            gobject: Loading::new(title, description).gobject().clone(),
        }
    }

    // Getters

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
