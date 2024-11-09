mod close;
mod debug;

use close::Close;
use debug::Debug;

use gtk::{
    gio::SimpleActionGroup,
    glib::{gformat, uuid_string_random, GString},
    prelude::{IsA, WidgetExt},
    Window,
};

pub struct Action {
    // Actions
    close: Close,
    debug: Debug,
    // Group
    gobject: SimpleActionGroup,
    name: GString,
}

impl Action {
    // Constructors

    /// Create **activated** [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) set
    /// with new [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html)
    /// for given [Window](https://docs.gtk.org/gtk4/class.Window.html)
    /// * useful for object-oriented work with GTK `detailed_name`, e.g. on GTK [Menu](https://docs.gtk.org/gio/class.Menu.html) build
    /// * this implementation also encapsulates `GObject` to prevent unexpected assignments
    /// * children actions implemented as wrapper also, that extend default [Variant](https://docs.gtk.org/glib/struct.Variant.html) features, etc
    pub fn new_for(window: &(impl IsA<Window> + WidgetExt)) -> Self {
        // Init group
        let name = uuid_string_random();
        let gobject = SimpleActionGroup::new();

        // Add group to window
        window.insert_action_group(&name, Some(&gobject));

        // Init actions
        let close = Close::new_for(&gobject, window.clone());
        let debug = Debug::new_for(&gobject, window.clone());

        Self {
            close,
            debug,
            gobject,
            name,
        }
    }

    // Getters

    pub fn debug(&self) -> GString {
        self.detailed_name(self.debug.name())
    }

    pub fn close(&self) -> GString {
        self.detailed_name(self.close.name())
    }

    // Helpers

    fn detailed_name(&self, action_name: GString) -> GString {
        gformat!("{}.{}", self.name, action_name)
    }
}
