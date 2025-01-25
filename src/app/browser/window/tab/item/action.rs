mod history;
mod ident;
mod load;

use history::History;
use ident::Ident;
use load::Load;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    pub history: Rc<History>,
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
        let ident = Rc::new(Ident::new());
        let load = Rc::new(Load::new());

        let history = Rc::new(History::build({
            let load = load.clone();
            move |request| load.activate(Some(&request), false)
        }));

        Self {
            history,
            ident,
            load,
        }
    }
}
