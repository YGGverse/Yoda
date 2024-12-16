mod close;
mod input;
mod match_case;
mod navigation;
mod tag;

use input::Input;
use navigation::Navigation;
use tag::Tag;

use gtk::{
    prelude::{BoxExt, ButtonExt, CheckButtonExt, EditableExt, TextBufferExt},
    Align, Box, Button, Orientation, TextBuffer, TextIter, TextSearchFlags, TextTag,
};
use std::rc::Rc;

pub struct Search {
    pub close: Button,
    pub g_box: Box,
    pub input: Rc<Input>,
    pub navigation: Rc<Navigation>,
}

impl Search {
    // Construct
    pub fn new(text_buffer: &TextBuffer) -> Self {
        // Init components
        let close = close::new();
        let input = Rc::new(Input::new());
        let match_case = match_case::new();
        let navigation = Rc::new(Navigation::new());
        let tag = Rc::new(Tag::new(text_buffer.tag_table()));

        // Init main container
        let g_box = Box::builder()
            .css_classes(["osd"])
            .orientation(Orientation::Horizontal)
            .valign(Align::Center)
            .vexpand(false)
            .visible(false)
            .build();

        g_box.append(&input.entry);
        g_box.append(&navigation.g_box);
        g_box.append(&match_case);
        g_box.append(&close);

        // Connect events
        close.connect_clicked({
            let input = input.clone();
            move |_| input.clean()
        });

        input.entry.connect_changed({
            let input = input.clone();
            let match_case = match_case.clone();
            let navigation = navigation.clone();
            let tag = tag.clone();
            let text_buffer = text_buffer.clone();
            move |_| {
                navigation.update(find(
                    &text_buffer,
                    &tag.found,
                    input.entry.text().as_str(),
                    match_case.is_active(),
                ));
                input.update(navigation.is_match());
            }
        });

        match_case.connect_toggled({
            let input = input.clone();
            let navigation = navigation.clone();
            let tag = tag.clone();
            let text_buffer = text_buffer.clone();
            move |this| {
                navigation.update(find(
                    &text_buffer,
                    &tag.found,
                    input.entry.text().as_str(),
                    this.is_active(),
                ));
                input.update(navigation.is_match());
            }
        });

        // Done
        Self {
            close,
            g_box,
            input,
            navigation,
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
