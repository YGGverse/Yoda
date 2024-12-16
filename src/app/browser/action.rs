mod about;
mod close;
mod debug;
mod escape;
mod profile;
mod update;

use about::About;
use close::Close;
use debug::Debug;
use escape::Escape;
use profile::Profile;
use update::Update;

use gtk::{
    gio::SimpleActionGroup,
    glib::{uuid_string_random, GString},
    prelude::ActionMapExt,
};
use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    // Actions
    pub about: Rc<About>,
    pub close: Rc<Close>,
    pub debug: Rc<Debug>,
    pub escape: Rc<Escape>,
    pub profile: Rc<Profile>,
    pub update: Rc<Update>,
    // Group
    pub id: GString,
    pub simple_action_group: SimpleActionGroup,
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init actions
        let about = Rc::new(About::new());
        let close = Rc::new(Close::new());
        let debug = Rc::new(Debug::new());
        let escape = Rc::new(Escape::new());
        let profile = Rc::new(Profile::new());
        let update = Rc::new(Update::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let simple_action_group = SimpleActionGroup::new();

        // Add action to given group
        simple_action_group.add_action(&about.simple_action);
        simple_action_group.add_action(&close.simple_action);
        simple_action_group.add_action(&debug.simple_action);
        simple_action_group.add_action(&escape.simple_action);
        simple_action_group.add_action(&profile.simple_action);
        simple_action_group.add_action(&update.simple_action);

        // Done
        Self {
            about,
            close,
            debug,
            escape,
            profile,
            update,
            id,
            simple_action_group,
        }
    }
}
