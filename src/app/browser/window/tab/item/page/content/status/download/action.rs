mod cancel;
mod complete;
mod update;

use cancel::Cancel;
use complete::Complete;
use update::Update;

use std::rc::Rc;

/// Callback API for `Download` widget
pub struct Action {
    pub cancel: Rc<Cancel>,
    pub complete: Rc<Complete>,
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
            cancel: Rc::new(Cancel::new()),
            complete: Rc::new(Complete::new()),
            update: Rc::new(Update::new()),
        }
    }
}
