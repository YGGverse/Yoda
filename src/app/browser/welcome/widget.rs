use adw::{
    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
    AlertDialog, ApplicationWindow, ResponseAppearance,
};
use gtk::prelude::GtkWindowExt;
use std::cell::RefCell;

const HEADING: &str = "Welcome!";
const BODY: &str = "Select profile for browser data";
const RESPONSE_QUIT: (&str, &str) = ("quit", "Quit");
const RESPONSE_CREATE: (&str, &str) = ("create", "Create new profile");

pub struct Widget {
    gobject: AlertDialog,
    parent: ApplicationWindow,
    responses: RefCell<Vec<String>>, // wanted to cleanup previous preset by key
}

impl Widget {
    // Constructors

    /// Create new `Self` for [Window](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1.3/class.ApplicationWindow.html)
    pub fn new(parent: ApplicationWindow) -> Self {
        // Init gobject
        let gobject = AlertDialog::builder()
            .heading(HEADING)
            .body(BODY)
            .close_response(RESPONSE_QUIT.0)
            .default_response(RESPONSE_CREATE.0)
            .build();

        // Set response variants
        gobject.add_responses(&[RESPONSE_QUIT, RESPONSE_CREATE]);

        // Decorate default response preset
        gobject.set_response_appearance(RESPONSE_CREATE.0, ResponseAppearance::Suggested);
        gobject.set_response_appearance(RESPONSE_QUIT.0, ResponseAppearance::Destructive);

        // Return new `Self`
        Self {
            gobject,
            parent,
            responses: RefCell::new(Vec::new()),
        }
    }

    // Actions

    /// Wrapper for default [response](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/signal.AlertDialog.response.html) signal
    /// * return profile ID, new record request on `None` or immediately close `self.parent` given on construction
    pub fn connect_response(&self, callback: impl Fn(Option<i64>) + 'static) {
        self.gobject.connect_response(None, {
            let parent = self.parent.clone();
            move |_, response| match response {
                id if id == RESPONSE_CREATE.0 => callback(None),
                id if id == RESPONSE_QUIT.0 => parent.close(),
                _ => callback(Some(response.parse::<i64>().unwrap())),
            }
        });
    }

    /// Show dialog with new profile responses preset
    pub fn present(&self, profiles: Vec<(String, String)>) {
        // Borrow current index to update
        let mut index = self.responses.borrow_mut();

        // Remove previous responses from widget
        for response in index.iter() {
            self.gobject.remove_response(response)
        }

        // Reset index
        index.clear();

        // Build new preset
        for (id, label) in profiles {
            self.gobject.add_response(&id, &label);
            index.push(id)
        }

        // Show dialog
        self.gobject.present(Some(&self.parent))
    }
}
