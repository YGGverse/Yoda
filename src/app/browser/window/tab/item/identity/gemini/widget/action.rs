mod update;
use update::Update;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper
pub struct Action {
    pub update: Rc<Update>,
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            update: Rc::new(Update::new()),
        }
    }
}
