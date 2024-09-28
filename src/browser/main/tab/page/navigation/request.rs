use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri, UriFlags},
    prelude::{ActionExt, EditableExt, EntryExt},
    Entry,
};

use std::sync::Arc;

pub struct Request {
    widget: Entry,
}

impl Request {
    // Construct
    pub fn new(
        text: Option<GString>,
        action_update: Arc<SimpleAction>,
        action_tab_page_reload: Arc<SimpleAction>,
    ) -> Self {
        // GTK
        let widget = Entry::builder()
            .placeholder_text("URL or search term...")
            .hexpand(true)
            .progress_fraction(0.0)
            .progress_pulse_step(0.1)
            .text(match text {
                Some(text) => text,
                None => GString::new(),
            })
            .build();

        // Connect events
        widget.connect_changed(move |_| {
            action_update.activate(None);
        });

        widget.connect_activate(move |_| {
            action_tab_page_reload.activate(None);
        });

        // Result
        Self { widget }
    }

    // Actions
    pub fn update(&self, progress_fraction: f64) {
        self.widget.set_progress_fraction(progress_fraction);
        // @TODO animate progress fraction
    }

    // Setters
    pub fn set_text(&self, value: &GString, activate: bool) {
        self.widget.set_text(value);

        if activate {
            self.widget.emit_activate();
        }
    }

    // Getters
    pub fn widget(&self) -> &Entry {
        &self.widget
    }

    pub fn is_empty(&self) -> bool {
        0 == self.widget.text_length()
    }

    pub fn text(&self) -> GString {
        self.widget.text()
    }

    pub fn uri(&self) -> Option<Uri> {
        match Uri::parse(&self.widget.text(), UriFlags::NONE) {
            Ok(uri) => Some(uri),
            _ => None,
        }
    }
}
