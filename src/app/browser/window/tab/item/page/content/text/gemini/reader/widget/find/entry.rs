use super::MARGIN;
use gtk::Entry;

pub fn new() -> Entry {
    Entry::builder()
        .hexpand(true)
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .placeholder_text("Find in text..")
        .primary_icon_activatable(false)
        .primary_icon_sensitive(false)
        .primary_icon_name("system-search-symbolic")
        .build()
}
