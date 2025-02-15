use gtk::{
    gio::{Cancellable, SimpleAction},
    glib::SignalHandlerId,
    prelude::{ActionExt, ToVariant},
};
use std::cell::RefCell;

// Defaults

/// C-compatible variant type
const DEFAULT_STATE: i32 = -1;

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Open` action
pub struct Open {
    cancellable: RefCell<Cancellable>,
    pub simple_action: SimpleAction,
}

impl Default for Open {
    fn default() -> Self {
        Self::new()
    }
}

impl Open {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            cancellable: RefCell::new(Cancellable::new()),
            simple_action: SimpleAction::new_stateful(
                &gtk::glib::uuid_string_random(),
                None,
                &DEFAULT_STATE.to_variant(),
            ),
        }
    }

    // Actions

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

    // Actions

    /// Formatted action connector for external implementation
    pub fn on_activate(&self, callback: impl Fn(Option<i32>, &str) + 'static) -> SignalHandlerId {
        use gtk::{prelude::FileExt, FileDialog, Window};
        use std::rc::Rc;

        let cancellable = self.cancellable();
        let callback = Rc::new(callback);

        self.simple_action.connect_activate(move |this, _| {
            let state = this
                .state()
                .expect("State value required")
                .get::<i32>()
                .expect("Parameter type does not match `i32`");

            FileDialog::builder()
                .build()
                .open(Window::NONE, Some(&cancellable), {
                    let callback = callback.clone();
                    move |result| {
                        if let Ok(file) = result {
                            callback(
                                if state == DEFAULT_STATE {
                                    None
                                } else {
                                    Some(state)
                                },
                                file.path().unwrap().to_str().unwrap(),
                            )
                        }
                    }
                });
        })
    }

    // Tools

    /// Gent new `Cancellable` by cancel previous one
    fn cancellable(&self) -> Cancellable {
        use gtk::prelude::CancellableExt;

        let cancellable = self.cancellable.replace(Cancellable::new());
        if !cancellable.is_cancelled() {
            cancellable.cancel();
        }

        self.cancellable.borrow().clone()
    }
}
