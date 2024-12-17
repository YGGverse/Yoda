mod input;
mod match_case;
mod navigation;

use super::Subject;
use input::Input;
use navigation::Navigation;

use gtk::{
    prelude::{
        BoxExt, ButtonExt, CheckButtonExt, EditableExt, TextBufferExt, TextViewExt, WidgetExt,
    },
    Align, Box, Orientation, TextIter, TextSearchFlags,
};
use std::{cell::RefCell, rc::Rc};

pub struct Form {
    pub input: Rc<Input>,
    pub g_box: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(subject: &Rc<RefCell<Option<Subject>>>) -> Self {
        // Init components
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

        // Connect events
        input.entry.connect_changed({
            let input = input.clone();
            let match_case = match_case.clone();
            let navigation = navigation.clone();
            let subject = subject.clone();
            move |_| {
                let matches = find(
                    &subject,
                    input.entry.text().as_str(),
                    match_case.is_active(),
                );
                input.update(!matches.is_empty());
                navigation.update(matches);
            }
        });

        match_case.connect_toggled({
            let input = input.clone();
            let navigation = navigation.clone();
            let subject = subject.clone();
            move |this| {
                let matches = find(&subject, input.entry.text().as_str(), this.is_active());
                input.update(!matches.is_empty());
                navigation.update(matches);
            }
        });

        navigation.back.button.connect_clicked({
            let subject = subject.clone();
            let navigation = navigation.clone();
            move |_| match subject.borrow().as_ref() {
                Some(subject) => match navigation.back(subject) {
                    Some((mut start, _)) => {
                        subject
                            .text_view
                            .scroll_to_iter(&mut start, 0.0, true, 0.0, 0.0);
                    }
                    None => todo!(),
                },
                None => todo!(),
            }
        });

        navigation.forward.button.connect_clicked({
            let subject = subject.clone();
            let navigation = navigation.clone();
            move |_| match subject.borrow().as_ref() {
                Some(subject) => match navigation.forward(subject) {
                    Some((mut start, _)) => {
                        subject
                            .text_view
                            .scroll_to_iter(&mut start, 0.0, true, 0.0, 0.0);
                    }
                    None => todo!(),
                },
                None => todo!(),
            }
        });

        // Done
        Self { g_box, input }
    }

    // Actions

    pub fn clean(&self) {
        self.input.clean();
    }

    pub fn show(&self) {
        self.g_box.set_visible(true);
        self.input.entry.grab_focus();
    }

    pub fn hide(&self) {
        self.g_box.set_visible(false)
    }
}

// Tools

fn find(
    subject: &Rc<RefCell<Option<Subject>>>,
    request: &str,
    is_match_case: bool,
) -> Vec<(TextIter, TextIter)> {
    // Init matches holder
    let mut result = Vec::new();

    // Borrow buffer
    match subject.borrow().as_ref() {
        Some(subject) => {
            // Get iters
            let (buffer_start, buffer_end) = subject.text_view.buffer().bounds();

            // Cleanup previous search highlights
            subject
                .text_view
                .buffer()
                .remove_tag(&subject.tag.current, &buffer_start, &buffer_end);
            subject
                .text_view
                .buffer()
                .remove_tag(&subject.tag.found, &buffer_start, &buffer_end);

            // Begin new search
            let mut next = buffer_start;
            while let Some((match_start, match_end)) = next.forward_search(
                request,
                match is_match_case {
                    true => TextSearchFlags::TEXT_ONLY,
                    false => TextSearchFlags::CASE_INSENSITIVE,
                },
                None, // unlimited
            ) {
                subject
                    .text_view
                    .buffer()
                    .apply_tag(&subject.tag.found, &match_start, &match_end);
                next = match_end;
                result.push((match_start, match_end));
            }
            result
        }
        None => todo!(), // unexpected
    }
}
