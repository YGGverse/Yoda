use gtk::{
    gio::SimpleAction,
    glib::uuid_string_random,
    prelude::{ActionExt, ToVariant},
};

// Defaults

/// C-compatible variant type
const DEFAULT_STATE: i32 = -1;

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `HistoryBack` action of `Window` group
pub struct HistoryBack {
    pub gobject: SimpleAction,
}

impl HistoryBack {
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

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn activate(&self) {
        self.gobject.activate(None);
    }

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
        self.gobject.connect_activate(move |this, _| {
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
