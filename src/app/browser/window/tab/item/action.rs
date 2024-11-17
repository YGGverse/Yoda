mod ident;
mod load;

use ident::Ident;
use load::Load;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    ident: Rc<Ident>,
    load: Rc<Load>,
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

    // Getters

    /// Get reference to `Ident` action
    pub fn ident(&self) -> &Rc<Ident> {
        &self.ident
    }

    /// Get reference to `Load` action
    pub fn load(&self) -> &Rc<Load> {
        &self.load
    }
}
