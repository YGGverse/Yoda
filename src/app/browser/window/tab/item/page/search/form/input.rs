use gtk::{
    prelude::{EditableExt, EntryExt, WidgetExt},
    Align, Entry, EntryIconPosition,
};

const MARGIN: i32 = 6;
const WIDTH_REQUEST: i32 = 280;

pub struct Input {
    pub entry: Entry,
}

impl Input {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init widget
        let entry = Entry::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .placeholder_text("Find in text..")
            .primary_icon_activatable(false)
            .primary_icon_name("system-search-symbolic")
            .primary_icon_sensitive(false)
            .valign(Align::Center)
            .vexpand(false)
            .width_request(WIDTH_REQUEST) // | .hexpand(true)
            .build();

        // Connect events
        entry.connect_icon_release(|this, position| match position {
            EntryIconPosition::Secondary => clean(this),
            _ => todo!(), // unexpected
        });

        entry.connect_changed(|this| {
            // toggle entry clear button
            if this.text().is_empty() {
                this.set_secondary_icon_name(None);
            } else {
                this.set_secondary_icon_name(Some("edit-clear-symbolic"));
            }
        });

        // Done
        Self { entry }
    }

    // Actions

    pub fn clean(&self) {
        clean(&self.entry)
    }

    pub fn update(&self, is_match: bool) {
        if is_match {
            self.entry.remove_css_class("error");
        } else {
            self.entry.add_css_class("error");
        }
    }
}

fn clean(entry: &Entry) {
    entry.delete_text(0, -1)
}
