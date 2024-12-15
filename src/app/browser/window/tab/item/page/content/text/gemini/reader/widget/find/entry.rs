use super::MARGIN;
use gtk::{
    prelude::{EditableExt, EntryExt},
    Entry, EntryIconPosition,
};

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

    // Done
    entry
}
