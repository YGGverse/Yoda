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
use std::{cell::Cell, rc::Rc};

const MARGIN: i32 = 6;

pub struct Find {
    pub close: Button,
    pub entry: Entry,
    pub g_box: Box,
}

impl Find {
    // Construct
    pub fn new(text_buffer: &TextBuffer) -> Self {
        // Init shared matches holder
        let matches = Rc::new(Cell::new(Vec::<(TextIter, TextIter)>::new()));

        // Init components
        let close = close::new();
        let entry = entry::new();
        let match_case = match_case::new();
        let navigation = Navigation::new();
        let tag = Tag::new(text_buffer.tag_table());

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
            let back = navigation.back.clone();
            let entry = entry.clone();
            let forward = navigation.forward.clone();
            let found_tag = tag.found.clone();
            let match_case = match_case.clone();
            let matches = matches.clone();
            let text_buffer = text_buffer.clone();
            move |_| {
                // do search
                let result = find(
                    &text_buffer,
                    &found_tag,
                    entry.text().as_str(),
                    match_case.is_active(),
                );

                // update components
                update(&entry, &back, &forward, result.is_empty());

                // update matches index
                matches.replace(result);
            }
        });

        match_case.connect_toggled({
            let entry = entry.clone();
            let found_tag = tag.found.clone();
            let matches = matches.clone();
            let text_buffer = text_buffer.clone();
            move |this| {
                // do search
                let result = find(
                    &text_buffer,
                    &found_tag,
                    entry.text().as_str(),
                    this.is_active(),
                );

                // update components
                update(
                    &entry,
                    &navigation.back,
                    &navigation.forward,
                    result.is_empty(),
                );

                // update matches index
                matches.replace(result);
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

fn update(entry: &Entry, back: &Button, forward: &Button, is_empty: bool) {
    if is_empty {
        entry.add_css_class("error");
        back.set_sensitive(false);
        forward.set_sensitive(false);
    } else {
        entry.remove_css_class("error");
        back.set_sensitive(false);
        forward.set_sensitive(true);
    }
}
