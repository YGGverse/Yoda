mod open;

use open::Open;

use gtk::{
    gio::SimpleActionGroup,
    glib::{uuid_string_random, GString},
    prelude::ActionMapExt,
};
use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    // Actions
    open: Rc<Open>,
    // Group
    id: GString,
    gobject: SimpleActionGroup,
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init actions
        let open = Rc::new(Open::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let gobject = SimpleActionGroup::new();

        // Add action to given group
        gobject.add_action(open.gobject());

        // Done
        Self { open, id, gobject }
    }

    // Getters

    /// Get reference `Open` action
    pub fn open(&self) -> &Rc<Open> {
        &self.open
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
