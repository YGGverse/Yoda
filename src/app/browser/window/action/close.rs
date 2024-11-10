use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, ToVariant},
};

// Defaults

/// C-compatible variant type
const DEFAULT_STATE: i32 = -1;

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Close` action of `Window` group
pub struct Close {
    gobject: SimpleAction,
}

impl Close {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gobject: SimpleAction::new_stateful(
                &uuid_string_random(),
                None,
                &DEFAULT_STATE.to_variant(),
            ),
        }
    }

    // Actions

    /* @TODO not in use
    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn activate(&self) {
        self.gobject.activate(None);
    } */

    /// Change action [state](https://docs.gtk.org/gio/method.SimpleAction.set_state.html)
    /// * set `DEFAULT_STATE` on `None`
    pub fn change_state(&self, state: Option<i32>) {
        self.gobject.change_state(
            &match state {
                Some(value) => value,
                None => DEFAULT_STATE,
            }
            .to_variant(),
        )
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn(Option<i32>) + 'static) {
        let state = self.state();
        self.gobject.connect_activate(move |_, _| callback(state));
    }

    // Getters

    pub fn state(&self) -> Option<i32> {
        let state = self
            .gobject
            .state()
            .expect("State value required")
            .get::<i32>()
            .expect("Parameter type does not match `i32`");

        if state != DEFAULT_STATE {
            Some(state)
        } else {
            None
        }
    }

    /// Get reference to [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) GObject
    pub fn gobject(&self) -> &SimpleAction {
        &self.gobject
    }

    /// Get auto-generated [action name](https://docs.gtk.org/gio/property.SimpleAction.name.html)
    pub fn id(&self) -> GString {
        self.gobject.name()
    }
}
