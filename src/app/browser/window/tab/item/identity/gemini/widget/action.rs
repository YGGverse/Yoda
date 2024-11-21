mod update;
use update::Update;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper
pub struct Action {
    pub update: Rc<Update>,
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
