mod input;
mod match_case;
mod navigation;
mod result;
mod separator;

use super::Subject;
use input::Input;
use navigation::Navigation;
use result::Result;

use gtk::{
    prelude::{
        BoxExt, ButtonExt, CheckButtonExt, DisplayExt, EditableExt, EntryExt, TextBufferExt,
        TextViewExt, WidgetExt,
    },
    Align, Box, Orientation, TextIter, TextSearchFlags, TextView,
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
        let result = Rc::new(Result::new());
        let input = Rc::new(Input::new());
        let match_case = match_case::new();
        let navigation = Rc::new(Navigation::new());
        let separator = separator::new();

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
        g_box.append(&separator);
        g_box.append(&result.label);

        // Connect events
        input.entry.connect_changed({
            let input = input.clone();
            let match_case = match_case.clone();
            let navigation = navigation.clone();
            let result = result.clone();
            let separator = separator.clone();
            let subject = subject.clone();
            move |this| {
                navigation.renew(find(
                    subject.borrow().as_ref().unwrap(), // @TODO handle
                    input.entry.text().as_str(),
                    match_case.is_active(),
                ));
                if !this.text().is_empty() {
                    input.update(navigation.total() > 0);
                    result.label.set_visible(true);
                    result.update(navigation.position(), navigation.total());
                    separator.set_visible(true);
                } else {
                    input.update(true);
                    result.label.set_visible(false);
                    separator.set_visible(false);
                }
            }
        });

        input.entry.connect_activate({
            let navigation = navigation.clone();
            let result = result.clone();
            let subject = subject.clone();
            move |this| match subject.borrow().as_ref() {
                Some(subject) => {
                    if !this.text().is_empty() {
                        match navigation.forward(subject) {
                            Some((mut start, _)) => {
                                result.update(navigation.position(), navigation.total());
                                scroll_to_iter(&subject.text_view, &mut start)
                            }
                            None => todo!(), // unexpected
                        }
                    }
                }
                None => todo!(),
            }
        });

        match_case.connect_toggled({
            let input = input.clone();
            let input = input.clone();
            let navigation = navigation.clone();
            let result = result.clone();
            let subject = subject.clone();
            move |this| {
                navigation.renew(find(
                    subject.borrow().as_ref().unwrap(), // @TODO handle
                    input.entry.text().as_str(),
                    this.is_active(),
                ));
                if !input.entry.text().is_empty() {
                    input.update(navigation.total() > 0);
                    result.label.set_visible(true);
                    result.update(navigation.position(), navigation.total());
                    separator.set_visible(true);
                } else {
                    input.update(true);
                    result.label.set_visible(false);
                    separator.set_visible(false);
                }
            }
        });

        navigation.back.button.connect_clicked({
            let navigation = navigation.clone();
            let result = result.clone();
            let subject = subject.clone();
            move |_| match subject.borrow().as_ref() {
                Some(subject) => {
                    match navigation.back(subject) {
                        Some((mut start, _)) => {
                            result.update(navigation.position(), navigation.total());
                            scroll_to_iter(&subject.text_view, &mut start)
                        }
                        None => todo!(), // unexpected
                    }
                }
                None => todo!(),
            }
        });

        navigation.forward.button.connect_clicked({
            let navigation = navigation.clone();
            let result = result.clone();
            let subject = subject.clone();
            move |_| match subject.borrow().as_ref() {
                Some(subject) => match navigation.forward(subject) {
                    Some((mut start, _)) => {
                        result.update(navigation.position(), navigation.total());
                        scroll_to_iter(&subject.text_view, &mut start)
                    }
                    None => todo!(), // unexpected
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

fn find(subject: &Subject, request: &str, is_match_case: bool) -> Vec<(TextIter, TextIter)> {
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

    // Init matches holder
    let mut result = Vec::new();

    // Skip search for empty request
    if request.is_empty() {
        return result;
    }

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

    if result.is_empty() {
        subject.text_view.display().beep()
    }

    result
}

fn scroll_to_iter(text_view: &TextView, iter: &mut TextIter) {
    text_view.scroll_to_iter(iter, 0.0, true, 0.0, 0.0);
}
