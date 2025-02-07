mod form;

use super::{Control, Header};
use gtk::{
    glib::{Bytes, GString},
    prelude::{TextBufferExt, TextViewExt},
    TextBuffer, TextView,
};
use std::{cell::RefCell, rc::Rc};

pub struct Text {
    header: Rc<RefCell<Header>>,
    pub text_view: TextView,
}

impl Text {
    // Constructors

    /// Build new `Self`
    pub fn build(control: &Rc<Control>) -> Self {
        use form::Form;

        // Init components
        let header = Rc::new(RefCell::new(Header {
            mime: Some("text/plain".into()), // some servers may reject request without MIME @TODO optional defaults
            token: None,
        }));

        // Init main widget
        let text_view = TextView::form();

        text_view.buffer().connect_changed({
            let control = control.clone();
            move |text_buffer| {
                control.update(
                    Some(gstring(text_buffer).len()),
                    Some(text_buffer.char_count()),
                )
            }
        });

        Self { header, text_view }
    }

    // Getters

    /// Get `Header` copy
    /// * borrow, do not take to have form re-send ability
    pub fn header(&self) -> Header {
        self.header.borrow().clone()
    }

    pub fn bytes(&self) -> Bytes {
        Bytes::from(self.gstring().as_bytes())
    }

    pub fn gstring(&self) -> GString {
        gstring(&self.text_view.buffer())
    }

    pub fn count(&self) -> i32 {
        self.text_view.buffer().char_count()
    }

    pub fn len(&self) -> usize {
        self.gstring().len()
    }

    // Setters

    /// Replace current `Header`
    /// * return previous object
    pub fn set_header(&self, header: Header) -> Header {
        self.header.replace(header)
    }
}

// Tools

fn gstring(text_buffer: &TextBuffer) -> GString {
    text_buffer.text(&text_buffer.start_iter(), &text_buffer.end_iter(), true)
}
