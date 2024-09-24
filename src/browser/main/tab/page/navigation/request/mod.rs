use gtk::{
    glib::GString,
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
            .progress_fraction(0.0)
            .progress_pulse_step(0.1)
            .build();

        // Connect events
        widget.connect_changed(|entry| {
            let _ = entry.activate_action("win.update", None);
        });

        widget.connect_activate(|entry| {
            let _ = entry.activate_action("win.tab_page_reload", None); // @TODO variant
        });

        // Result
        Self { widget }
    }

    // Actions
    pub fn update(&self) {
        // @TODO animate progress fraction
    }

    // Setters
    pub fn set_text(&self, value: &GString, activate: bool) {
        self.widget.set_text(value);

        if activate {
            self.widget.activate();
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
}
