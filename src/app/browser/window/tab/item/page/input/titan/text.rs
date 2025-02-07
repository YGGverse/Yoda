mod form;

use super::Control;
use gtk::{
    glib::{Bytes, GString},
    prelude::{TextBufferExt, TextViewExt},
    TextView,
};
use std::rc::Rc;

pub trait Text {
    fn text(control: &Rc<Control>) -> Self;
    fn to_bytes(&self) -> Bytes;
    fn to_gstring(&self) -> GString;
    fn len(&self) -> usize;
    fn count(&self) -> i32;
}

impl Text for TextView {
    fn text(control: &Rc<Control>) -> Self {
        use form::Form;

        let text_view = TextView::form();

        text_view.buffer().connect_changed({
            let control = control.clone();
            let text_view = text_view.clone();
            move |text_buffer| control.update(Some(text_view.len()), Some(text_buffer.char_count()))
        });

        text_view
    }

    fn to_bytes(&self) -> Bytes {
        Bytes::from(self.to_gstring().as_bytes())
    }

    fn to_gstring(&self) -> GString {
        let buffer = self.buffer();
        self.buffer()
            .text(&buffer.start_iter(), &buffer.end_iter(), true)
    }

    fn count(&self) -> i32 {
        self.buffer().char_count()
    }

    fn len(&self) -> usize {
        let buffer = self.buffer();

        buffer
            .text(&buffer.start_iter(), &buffer.end_iter(), true)
            .len()
    }
}
