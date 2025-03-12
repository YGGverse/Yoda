mod history;
mod home;
mod identity;
mod load;
mod reload;

use gtk::gio::SimpleAction;
use history::History;
use home::Home;
use identity::Identity;
use load::Load;
use reload::Reload;

use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    pub history: Rc<History>,
    pub home: SimpleAction,
    pub identity: SimpleAction,
    pub load: Rc<Load>,
    pub reload: SimpleAction,
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
        let home = SimpleAction::home();
        let identity = SimpleAction::identity();
        let load = Rc::new(Load::new());
        let reload = SimpleAction::reload();

        let history = Rc::new(History::build({
            let load = load.clone();
            move |request| load.activate(Some(&request))
        }));

        Self {
            history,
            home,
            identity,
            load,
            reload,
        }
    }
}
