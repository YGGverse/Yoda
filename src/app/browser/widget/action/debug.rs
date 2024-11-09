use gtk::{
    gio::{SimpleAction, SimpleActionGroup},
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, ActionMapExt, GtkWindowExt, IsA},
    Window,
};

pub struct Debug {
    gobject: SimpleAction,
}

impl Debug {
    // Constructors

    /// Create new [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html)
    /// for given [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html)
    /// and [Window](https://docs.gtk.org/gtk4/class.Window.html)
    /// * this constructor **activate** default feature
    pub fn new_for(group: &SimpleActionGroup, window: impl IsA<Window>) -> Self {
        // Init action GObject
        let gobject = SimpleAction::new(&uuid_string_random(), None);

        // Add action to given group
        group.add_action(&gobject);

        // Connect default feature on activate
        gobject.connect_activate(move |_, _| {
            window.emit_enable_debugging(true);
        });

        // Done
        Self { gobject }
    }

    // Getters

    pub fn name(&self) -> GString {
        self.gobject.name()
    }
}
