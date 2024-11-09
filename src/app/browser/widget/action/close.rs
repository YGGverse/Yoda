use gtk::{
    gio::{SimpleAction, SimpleActionGroup},
    prelude::{ActionMapExt, GtkWindowExt, IsA},
    Window,
};

pub struct Close {
    gobject: SimpleAction,
}

impl Close {
    // Constructors

    /// Create new [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html)
    /// for given [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html)
    /// and [Window](https://docs.gtk.org/gtk4/class.Window.html)
    /// * this constructor **activate** default feature
    pub fn new_for(group: &SimpleActionGroup, window: impl IsA<Window>) -> Self {
        // Get action config
        let (_group_name, action_name, parameter_type, _accels) =
            crate::action::APP_BROWSER_WIDGET_CLOSE;

        // Init action GObject
        let gobject = SimpleAction::new(&action_name, parameter_type);

        // Add action to given group
        group.add_action(&gobject);

        // Connect default feature on activate
        gobject.connect_activate(move |_, _| {
            window.close();
        });

        // Done
        Self { gobject }
    }
}
