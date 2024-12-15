mod close;
mod entry;
mod match_case;
mod navigation;
mod tag;

use navigation::Navigation;
use tag::Tag;

use gtk::{
    prelude::{BoxExt, ButtonExt, CheckButtonExt, EditableExt, TextBufferExt, WidgetExt},
    Box, Button, Entry, Orientation, TextBuffer, TextIter, TextSearchFlags, TextTag,
};
use std::rc::Rc;

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
        let close = close::new();
        let entry = entry::new();
        let match_case = match_case::new();
        let navigation = Rc::new(Navigation::new());
        let tag = Rc::new(Tag::new(text_buffer.tag_table()));

        // Init main container
        let g_box = Box::builder()
            // .css_classes(["app-notification"])
            .orientation(Orientation::Horizontal)
            .build();

        g_box.append(&entry);
        g_box.append(&navigation.g_box);
        g_box.append(&match_case);
        g_box.append(&close);

        // Connect events
        close.connect_clicked({
            let entry = entry.clone();
            move |_| entry.delete_text(0, -1)
        });

        entry.connect_changed({
            let entry = entry.clone();
            let match_case = match_case.clone();
            let navigation = navigation.clone();
            let tag = tag.clone();
            let text_buffer = text_buffer.clone();
            move |_| {
                navigation.update(find(
                    &text_buffer,
                    &tag.found,
                    entry.text().as_str(),
                    match_case.is_active(),
                ));
                update(&entry, &navigation);
            }
        });

        match_case.connect_toggled({
            let entry = entry.clone();
            let navigation = navigation.clone();
            let tag = tag.clone();
            let text_buffer = text_buffer.clone();
            move |this| {
                navigation.update(find(
                    &text_buffer,
                    &tag.found,
                    entry.text().as_str(),
                    this.is_active(),
                ));
                update(&entry, &navigation);
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

// Tools

fn find(
    text_buffer: &TextBuffer,
    found_tag: &TextTag,
    subject: &str,
    is_match_case: bool,
) -> Vec<(TextIter, TextIter)> {
    // Init start matches result
    let mut result = Vec::new();

    // Cleanup previous search results
    text_buffer.remove_tag(
        found_tag,
        &text_buffer.start_iter(),
        &text_buffer.end_iter(),
    );

    // Begin search
    let mut next = text_buffer.start_iter();
    while let Some((start, end)) = next.forward_search(
        subject,
        match is_match_case {
            true => TextSearchFlags::TEXT_ONLY,
            false => TextSearchFlags::CASE_INSENSITIVE,
        },
        None, // unlimited
    ) {
        text_buffer.apply_tag(found_tag, &start, &end);
        next = end;
        result.push((start, end));
    }
    result
}

fn update(entry: &Entry, navigation: &Rc<Navigation>) {
    if navigation.matches.take().is_empty() {
        entry.add_css_class("error");
        navigation.back.set_sensitive(false);
        navigation.forward.set_sensitive(false);
    } else {
        entry.remove_css_class("error");
        navigation.back.set_sensitive(false);
        navigation.forward.set_sensitive(true);
    }
}
