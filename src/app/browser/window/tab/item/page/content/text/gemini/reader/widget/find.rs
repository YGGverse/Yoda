use gtk::{
    gdk::Cursor,
    prelude::{BoxExt, EditableExt, EntryExt},
    Box, Button, Entry, EntryIconPosition, Orientation,
};

const MARGIN: i32 = 6;

pub struct Find {
    pub g_box: Box,
}

impl Find {
    // Construct
    pub fn new() -> Self {
        // Init components
        let close = Button::builder()
            .cursor(&Cursor::from_name("default", None).unwrap())
            .icon_name("window-close-symbolic")
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_top(MARGIN)
            .tooltip_text("Close find bar")
            .build();

        let entry = Entry::builder()
            .hexpand(true)
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .placeholder_text("Find in text..")
            .primary_icon_activatable(false)
            .primary_icon_name("system-search-symbolic")
            .build();

        // Init main container
        let g_box = Box::builder().orientation(Orientation::Horizontal).build();

        g_box.append(&entry);
        g_box.append(&close);

        // Connect events
        entry.connect_activate(|_| {}); // @TODO

        entry.connect_changed(move |this| {
            if this.text().is_empty() {
                this.set_secondary_icon_name(None);
            } else {
                this.set_secondary_icon_name(Some("edit-clear-symbolic"));
            }
        });

        entry.connect_icon_release(move |this, position| match position {
            EntryIconPosition::Secondary => this.delete_text(0, -1),
            _ => todo!(), // unexpected
        });

        // Done
        Self { g_box }
    }
}
