mod failure;

use failure::Failure;

use adw::StatusPage;

pub struct Status {
    gobject: StatusPage,
}

impl Status {
    // Construct
    pub fn new_error(title: &str, description: &str) -> Self {
        Self {
            gobject: Failure::new(title, description),
        }
    }

    // Getters
    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
