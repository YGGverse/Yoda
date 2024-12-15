use gtk::{
    gdk::{Cursor, RGBA},
    prelude::{BoxExt, ButtonExt, CheckButtonExt, EditableExt, EntryExt, TextBufferExt, WidgetExt},
    Box, Button, CheckButton, Entry, EntryIconPosition, Orientation, TextBuffer, TextSearchFlags,
    TextTag,
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
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .tooltip_text("Close find bar")
            .build();

        let match_case = CheckButton::builder()
            .cursor(&Cursor::from_name("default", None).unwrap())
            .label("Match case")
            .build();

        let navigation = Box::builder()
            .css_classes([
                "linked", // merge childs
            ])
            .margin_end(MARGIN)
            .orientation(Orientation::Horizontal)
            .build();

        let back = Button::builder()
            .icon_name("go-previous-symbolic")
            .margin_bottom(MARGIN)
            .margin_top(MARGIN)
            .sensitive(false)
            .tooltip_text("Back")
            .build();

        let forward = Button::builder()
            .icon_name("go-next-symbolic")
            .margin_bottom(MARGIN)
            .margin_top(MARGIN)
            .sensitive(false)
            .tooltip_text("Forward")
            .build();

        navigation.append(&back);
        navigation.append(&forward);

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

        let found_tag = TextTag::builder()
            .background_rgba(&RGBA::new(0.502, 0.502, 0.502, 0.5)) // @TODO
            .build();
        text_buffer.tag_table().add(&found_tag);

        // Init main container
        let g_box = Box::builder()
            // .css_classes(["app-notification"])
            .orientation(Orientation::Horizontal)
            .build();

        g_box.append(&entry);
        g_box.append(&navigation);
        g_box.append(&match_case);
        g_box.append(&close);

        // Connect events
        close.connect_clicked({
            let entry = entry.clone();
            move |_| entry.delete_text(0, -1)
        });

        entry.connect_changed({
            let entry = entry.clone();
            let found_tag = found_tag.clone();
            let match_case = match_case.clone();
            let text_buffer = text_buffer.clone();
            move |this| {
                // toggle clear action
                if this.text().is_empty() {
                    this.set_secondary_icon_name(None);
                } else {
                    this.set_secondary_icon_name(Some("edit-clear-symbolic"));
                }
                // apply changes
                if find(
                    &text_buffer,
                    &found_tag,
                    entry.text().as_str(),
                    match_case.is_active(),
                )
                .is_positive()
                {
                    entry.remove_css_class("error");
                } else {
                    entry.add_css_class("error");
                }
            }
        });

        entry.connect_icon_release(move |this, position| match position {
            EntryIconPosition::Secondary => this.delete_text(0, -1),
            _ => todo!(), // unexpected
        });

        match_case.connect_toggled({
            let entry = entry.clone();
            let found_tag = found_tag.clone();
            let text_buffer = text_buffer.clone();
            move |this| {
                if find(
                    &text_buffer,
                    &found_tag,
                    entry.text().as_str(),
                    this.is_active(),
                )
                .is_positive()
                {
                    entry.remove_css_class("error");
                } else {
                    entry.add_css_class("error");
                }
            }
        });

        // Done
        Self {
            close,
            entry,
            g_box,
        }
    }
}

fn find(text_buffer: &TextBuffer, found_tag: &TextTag, subject: &str, is_match_case: bool) -> i64 {
    // Cleanup previous search results
    text_buffer.remove_tag(
        found_tag,
        &text_buffer.start_iter(),
        &text_buffer.end_iter(),
    );

    // Begin search
    let mut next = text_buffer.start_iter();
    let mut total: i64 = 0;
    while let Some((start, end)) = next.forward_search(
        subject,
        match is_match_case {
            true => TextSearchFlags::TEXT_ONLY,
            false => TextSearchFlags::CASE_INSENSITIVE,
        },
        None, // unlimited
    ) {
        text_buffer.apply_tag(found_tag, &start, &end);
        total += 1;
        next = end;
    }

    total
}
