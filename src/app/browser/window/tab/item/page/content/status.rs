mod error;

use error::Error;

use adw::StatusPage;

pub struct Status {
    gobject: StatusPage,
}

impl Status {
    // Construct
    pub fn new_error(title: &str, description: &str) -> Self {
        Self {
            gobject: Error::new(title, description),
        }
    }

    // Getters
    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
