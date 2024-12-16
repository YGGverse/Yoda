mod close;
mod input;
mod match_case;
mod navigation;

use super::Buffer;
use input::Input;
use navigation::Navigation;

use gtk::{
    prelude::{BoxExt, ButtonExt, CheckButtonExt, EditableExt, TextBufferExt, WidgetExt},
    Align, Box, Orientation, TextIter, TextSearchFlags,
};
use std::{cell::RefCell, rc::Rc};

pub struct Form {
    pub g_box: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(buffer: &Rc<RefCell<Option<Buffer>>>) -> Self {
        // Init components
        let close = close::new();
        let input = Rc::new(Input::new());
        let match_case = match_case::new();
        let navigation = Rc::new(Navigation::new());

        // Init main container
        let g_box = Box::builder()
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
            let buffer = buffer.clone();
            move |_| {
                navigation.update(find(
                    &buffer,
                    input.entry.text().as_str(),
                    match_case.is_active(),
                ));
                input.update(navigation.is_match());
            }
        });

        match_case.connect_toggled({
            let input = input.clone();
            let navigation = navigation.clone();
            let buffer = buffer.clone();
            move |this| {
                navigation.update(find(&buffer, input.entry.text().as_str(), this.is_active()));
                input.update(navigation.is_match());
            }
        });

        // Done
        Self { g_box }
    }

    // Actions

    pub fn show(&self) {
        //self.buffer.get_mut().is_none()
        self.g_box.set_visible(true)
    }

    pub fn hide(&self) {
        self.g_box.set_visible(false)
    }

    pub fn toggle(&self) {
        if self.g_box.is_visible() {
            self.hide()
        } else {
            self.show()
        }
    }
}

// Tools

fn find(
    buffer: &Rc<RefCell<Option<Buffer>>>,
    subject: &str,
    is_match_case: bool,
) -> Vec<(TextIter, TextIter)> {
    // Init matches holder
    let mut result = Vec::new();

    // Borrow buffer
    match buffer.borrow().as_ref() {
        Some(buffer) => {
            // Get iters
            let buffer_start = buffer.text_buffer.start_iter();
            let buffer_end = buffer.text_buffer.end_iter();

            // Cleanup previous search highlights
            buffer
                .text_buffer
                .remove_tag(&buffer.tag.current, &buffer_start, &buffer_end);
            buffer
                .text_buffer
                .remove_tag(&buffer.tag.found, &buffer_start, &buffer_end);

            // Begin new search
            let mut next = buffer_start;
            while let Some((match_start, match_end)) = next.forward_search(
                subject,
                match is_match_case {
                    true => TextSearchFlags::TEXT_ONLY,
                    false => TextSearchFlags::CASE_INSENSITIVE,
                },
                None, // unlimited
            ) {
                buffer
                    .text_buffer
                    .apply_tag(&buffer.tag.found, &match_start, &match_end);
                next = match_end;
                result.push((match_start, match_end));
            }
            result
        }
        None => todo!(), // unexpected
    }
}
