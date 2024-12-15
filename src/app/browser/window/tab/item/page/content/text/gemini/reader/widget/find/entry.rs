use gtk::{
    prelude::{EditableExt, EntryExt},
    Entry, EntryIconPosition,
};

const MARGIN: i32 = 6;

pub fn new() -> Entry {
    // Init widget
    let entry = Entry::builder()
        .hexpand(true)
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .placeholder_text("Find in text..")
        .primary_icon_activatable(false)
        .primary_icon_sensitive(false)
        .primary_icon_name("system-search-symbolic")
        .build();

    // Connect events
    entry.connect_icon_release(|this, position| match position {
        EntryIconPosition::Secondary => this.delete_text(0, -1),
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
    entry
}
