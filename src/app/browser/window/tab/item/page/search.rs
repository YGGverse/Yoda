mod form;
mod placeholder;
mod subject;

use form::Form;
use placeholder::Placeholder;
use subject::Subject;

use gtk::{
    prelude::{BoxExt, WidgetExt},
    Align, Box, Orientation, TextBuffer, TextView,
};
use std::{cell::RefCell, rc::Rc};

pub struct Search {
    subject: Rc<RefCell<Option<Subject>>>,
    pub form: Rc<Form>,
    pub placeholder: Rc<Placeholder>,
    pub g_box: Box,
}

impl Search {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init components
        let subject = Rc::new(RefCell::new(None));
        let form = Rc::new(Form::new(&subject));
        let placeholder = Rc::new(Placeholder::new());

        // Init main container
        let g_box = Box::builder()
            .orientation(Orientation::Vertical)
            .valign(Align::Center)
            .vexpand(false)
            .visible(false)
            .build();

        g_box.append(&form.g_box);
        g_box.append(&placeholder.label);

        // Done
        Self {
            subject,
            form,
            g_box,
            placeholder,
        }
    }

    // Actions

    pub fn show(&self) {
        if self.subject.borrow().is_some() {
            self.form.show();
            self.placeholder.hide();
        } else {
            self.form.hide();
            self.placeholder.show();
        }
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

    /// * currently supports [TextView](https://docs.gtk.org/gtk4/class.TextView.html) only
    pub fn update(&self, text_view: Option<TextView>) {
        self.subject.replace(match text_view {
            Some(subject) => Some(Subject::new(subject)),
            None => None,
        });
    }
}
