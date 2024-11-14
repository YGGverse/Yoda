mod auth;
mod load;

use auth::Auth;
use load::Load;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    auth: Rc<Auth>,
    load: Rc<Load>,
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            auth: Rc::new(Auth::new()),
            load: Rc::new(Load::new()),
        }
    }

    // Getters

    /// Get reference to `Auth` action
    pub fn auth(&self) -> &Rc<Auth> {
        &self.auth
    }

    /// Get reference to `Load` action
    pub fn load(&self) -> &Rc<Load> {
        &self.load
    }
}
