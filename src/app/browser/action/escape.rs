use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, ToVariant},
};

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Escape` action of `Browser` group
pub struct Escape {
    pub simple_action: SimpleAction,
}

impl Default for Escape {
    fn default() -> Self {
        Self::new()
    }
}

impl Escape {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            simple_action: SimpleAction::new_stateful(
                &uuid_string_random(),
                None,
                &String::new().to_variant(),
            ),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// * this action reset previous state for action after activation
    pub fn activate_stateful_once(&self, tab_item_id: Option<GString>) {
        // Save current state in memory
        let _tab_item_id = state(&self.simple_action);

        // Apply requested state
        self.change_state(tab_item_id);

        // Activate action
        self.simple_action.activate(None);

        // Return previous state
        self.change_state(_tab_item_id);
    }

    /// Change action [state](https://docs.gtk.org/gio/method.SimpleAction.set_state.html)
    /// * set default state on `None`
    pub fn change_state(&self, state: Option<GString>) {
        self.simple_action.change_state(
            &match state {
                Some(value) => value.to_string(),
                None => String::new(),
            }
            .to_variant(),
        )
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    pub fn connect_activate(&self, callback: impl Fn(Option<GString>) + 'static) {
        self.simple_action
            .connect_activate(move |this, _| callback(state(this)));
    }
}

/// Shared helper to get C-based action state in Optional format
fn state(this: &SimpleAction) -> Option<GString> {
    let state = this
        .state()
        .expect("State value required")
        .get::<String>()
        .expect("Parameter type does not match `String`");

    if state.is_empty() {
        None
    } else {
        Some(state.into())
    }
}
