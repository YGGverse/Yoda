mod load;
use load::Load;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    // Actions
    load: Rc<Load>,
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            load: Rc::new(Load::new()),
        }
    }

    // Getters

    /// Get reference to `Load` action
    pub fn load(&self) -> &Rc<Load> {
        &self.load
    }
}
