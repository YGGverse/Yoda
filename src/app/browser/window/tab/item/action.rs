mod ident;
mod load;

use ident::Ident;
use load::Load;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    pub ident: Rc<Ident>,
    pub load: Rc<Load>,
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
            ident: Rc::new(Ident::new()),
            load: Rc::new(Load::new()),
        }
    }
}
