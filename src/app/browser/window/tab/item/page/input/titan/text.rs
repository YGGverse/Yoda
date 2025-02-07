mod control;
mod form;

use super::Header;
use gtk::glib::Bytes;

pub trait Text {
    fn text(callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static) -> Self;
}

impl Text for gtk::Box {
    fn text(callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static) -> Self {
        use control::{Control, Upload};
        use form::Form;
        use gtk::{
            prelude::{BoxExt, ButtonExt, TextBufferExt, TextViewExt},
            Orientation, TextView,
        };
        use std::{cell::Cell, rc::Rc};

        // Init components
        let header = Rc::new(Cell::new(Header {
            mime: Some("text/plain".into()), // some servers require not empty content type
            token: None,
        }));
        let control = Rc::new(Control::build(&header));
        let form = TextView::form();

        // Init widget
        let g_box = {
            const MARGIN: i32 = 8;
            let g_box = gtk::Box::builder()
                .margin_bottom(MARGIN / 2)
                .margin_end(MARGIN)
                .margin_start(MARGIN)
                .orientation(Orientation::Vertical)
                .spacing(MARGIN)
                .build();

            g_box.append(&form);
            g_box.append(&control.g_box);
            g_box
        };

        // Connect events

        form.buffer().connect_changed({
            let control = control.clone();
            move |this| {
                control.update(
                    this.char_count(),
                    this.text(&this.start_iter(), &this.end_iter(), true).len(),
                )
            }
        });

        control.upload.connect_clicked(move |this| {
            this.set_uploading();
            callback(
                header.take(),
                Bytes::from(form.text().as_bytes()),
                Box::new({
                    let this = this.clone();
                    move || this.set_resend() // on failure
                }),
            )
        });

        g_box
    }
}
