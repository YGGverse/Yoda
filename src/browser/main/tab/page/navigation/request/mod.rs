use gtk::{
    prelude::{EditableExt, EntryExt, WidgetExt},
    Entry,
};

pub struct Request {
    widget: Entry,
}

impl Request {
    // Construct
    pub fn new() -> Request {
        // GTK
        let widget = Entry::builder()
            .placeholder_text("URL or search term...")
            .hexpand(true)
            .progress_pulse_step(0.1)
            .build();

        // Connect events
        widget.connect_changed(|entry| {
            let _ = entry.activate_action("win.update", None); // @TODO
        });

        widget.connect_activate(|entry| {
            // @TODO
        });

        // Result
        Self { widget }
    }

    // Actions
    pub fn update(&self) {}

    // Getters
    pub fn widget(&self) -> &Entry {
        &self.widget
    }

    pub fn is_empty(&self) -> bool {
        0 == self.widget.text_length()
    }
}
