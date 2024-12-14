use gtk::{
    gio::SimpleAction,
    glib::uuid_string_random,
    prelude::{ActionExt, ToVariant},
};

// Defaults

/// C-compatible variant type
const DEFAULT_STATE: i32 = -1;

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Reload` action of `Window` group
pub struct Reload {
    pub simple_action: SimpleAction,
}

impl Reload {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            simple_action: SimpleAction::new_stateful(
                &uuid_string_random(),
                None,
                &DEFAULT_STATE.to_variant(),
            ),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn activate(&self) {
        self.simple_action.activate(None);
    }

    /// Change action [state](https://docs.gtk.org/gio/method.SimpleAction.set_state.html)
    /// * set `DEFAULT_STATE` on `None`
    pub fn change_state(&self, state: Option<i32>) {
        self.simple_action.change_state(
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
        self.simple_action.connect_activate(move |this, _| {
            let state = this
                .state()
                .expect("State value required")
                .get::<i32>()
                .expect("Parameter type does not match `i32`");

            callback(if state == DEFAULT_STATE {
                None
            } else {
                Some(state)
            })
        });
    }
}
