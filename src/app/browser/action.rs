mod about;
mod close;
mod debug;
mod profile;
mod update;

use about::About;
use close::Close;
use debug::Debug;
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
    about: Rc<About>,
    close: Rc<Close>,
    debug: Rc<Debug>,
    profile: Rc<Profile>,
    update: Rc<Update>,
    // Group
    id: GString,
    gobject: SimpleActionGroup,
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init actions
        let about = Rc::new(About::new());
        let close = Rc::new(Close::new());
        let debug = Rc::new(Debug::new());
        let profile = Rc::new(Profile::new());
        let update = Rc::new(Update::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let gobject = SimpleActionGroup::new();

        // Add action to given group
        gobject.add_action(about.gobject());
        gobject.add_action(close.gobject());
        gobject.add_action(debug.gobject());
        gobject.add_action(profile.gobject());
        gobject.add_action(update.gobject());

        // Done
        Self {
            about,
            close,
            debug,
            profile,
            update,
            id,
            gobject,
        }
    }

    // Getters

    /// Get reference `About` action
    pub fn about(&self) -> &Rc<About> {
        &self.about
    }

    /// Get reference `Close` action
    pub fn close(&self) -> &Rc<Close> {
        &self.close
    }

    /// Get reference `Debug` action
    pub fn debug(&self) -> &Rc<Debug> {
        &self.debug
    }

    /// Get reference `Profile` action
    pub fn profile(&self) -> &Rc<Profile> {
        &self.profile
    }

    /// Get reference `Update` action
    pub fn update(&self) -> &Rc<Update> {
        &self.update
    }

    /// Get auto-generated name for action group
    /// * useful for manual relationship with GObjects or as the `detailed_name`
    ///   for [Accels](https://docs.gtk.org/gtk4/method.Application.set_accels_for_action.html) or
    ///   [Menu](https://docs.gtk.org/gio/class.Menu.html) builder
    pub fn id(&self) -> &GString {
        &self.id
    }

    /// Get reference to [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) GObject
    pub fn gobject(&self) -> &SimpleActionGroup {
        &self.gobject
    }
}
