mod about;
mod close;
mod debug;
mod focus;
mod profile;
mod update;

use about::About;
use close::Close;
use debug::Debug;
use focus::Focus;
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
    pub focus: Rc<Focus>,
    pub profile: Rc<Profile>,
    pub update: Rc<Update>,
    // Group
    pub id: GString,
    pub gobject: SimpleActionGroup,
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init actions
        let about = Rc::new(About::new());
        let close = Rc::new(Close::new());
        let debug = Rc::new(Debug::new());
        let focus = Rc::new(Focus::new());
        let profile = Rc::new(Profile::new());
        let update = Rc::new(Update::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let gobject = SimpleActionGroup::new();

        // Add action to given group
        gobject.add_action(&about.gobject);
        gobject.add_action(&close.gobject);
        gobject.add_action(&debug.gobject);
        gobject.add_action(&focus.gobject);
        gobject.add_action(&profile.gobject);
        gobject.add_action(&update.gobject);

        // Done
        Self {
            about,
            close,
            debug,
            focus,
            profile,
            update,
            id,
            gobject,
        }
    }
}
