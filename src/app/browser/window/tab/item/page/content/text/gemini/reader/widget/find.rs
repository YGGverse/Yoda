use gtk::{
    gdk::{Cursor, RGBA},
    prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, TextBufferExt},
    Box, Button, Entry, EntryIconPosition, Orientation, TextBuffer, TextSearchFlags, TextTag,
};

const MARGIN: i32 = 6;

pub struct Find {
    pub close: Button,
    pub entry: Entry,
    pub g_box: Box,
}

impl Find {
    // Construct
    pub fn new(text_buffer: &TextBuffer) -> Self {
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

        let text_tag = TextTag::builder()
            .background_rgba(&RGBA::new(0.502, 0.502, 0.502, 0.5)) // @TODO
            .build();
        text_buffer.tag_table().add(&text_tag);

        // Init main container
        let g_box = Box::builder().orientation(Orientation::Horizontal).build();

        g_box.append(&entry);
        g_box.append(&close);

        // Connect events
        close.connect_clicked({
            let entry = entry.clone();
            move |_| entry.delete_text(0, -1)
        });

        entry.connect_changed({
            let entry = entry.clone();
            let text_buffer = text_buffer.clone();
            let text_tag = text_tag.clone();
            move |this| {
                // Toggle clear action
                if this.text().is_empty() {
                    this.set_secondary_icon_name(None);
                } else {
                    this.set_secondary_icon_name(Some("edit-clear-symbolic"));
                }

                // Cleanup previous search results
                text_buffer.remove_tag(
                    &text_tag,
                    &text_buffer.start_iter(),
                    &text_buffer.end_iter(),
                );

                // Get subject once
                let query = entry.text();

                // Begin search
                let mut next = text_buffer.start_iter();
                while let Some((start, end)) = next.forward_search(
                    &query,
                    TextSearchFlags::CASE_INSENSITIVE, // @TODO
                    None,                              // unlimited
                ) {
                    text_buffer.apply_tag(&text_tag, &start, &end);
                    next = end;
                }
            }
        });

        entry.connect_icon_release(move |this, position| match position {
            EntryIconPosition::Secondary => this.delete_text(0, -1),
            _ => todo!(), // unexpected
        });

        // Done
        Self {
            close,
            entry,
            g_box,
        }
    }
}
