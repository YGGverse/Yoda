mod about;
mod close;
mod debug;
mod escape;
mod profile;

use about::About;
use close::Close;
use debug::Debug;
use escape::Escape;
use profile::Profile;

use gtk::{
    gio::{SimpleAction, SimpleActionGroup},
    glib::{GString, uuid_string_random},
    prelude::ActionMapExt,
};
use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    // Actions
    pub about: Rc<About>,
    pub close: Rc<Close>,
    pub debug: Rc<Debug>,
    pub escape: SimpleAction,
    pub profile: Rc<Profile>,
    // Group
    pub id: GString,
    pub simple_action_group: SimpleActionGroup,
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
        // Init actions
        let about = Rc::new(About::new());
        let close = Rc::new(Close::new());
        let debug = Rc::new(Debug::new());
        let escape = SimpleAction::escape();
        let profile = Rc::new(Profile::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let simple_action_group = SimpleActionGroup::new();

        // Add action to given group
        simple_action_group.add_action(&about.simple_action);
        simple_action_group.add_action(&close.simple_action);
        simple_action_group.add_action(&debug.simple_action);
        simple_action_group.add_action(&escape);
        simple_action_group.add_action(&profile.simple_action);

        // Done
        Self {
            about,
            close,
            debug,
            escape,
            profile,
            id,
            simple_action_group,
        }
    }
}
