mod control;
mod form;

use super::Header;
use control::Control;
use control::Upload;
use form::Form;
use gtk::glib::Bytes;
use gtk::{
    prelude::{BoxExt, ButtonExt, TextBufferExt, TextViewExt},
    Orientation, TextView,
};
use std::cell::Cell;
use std::rc::Rc;

const MARGIN: i32 = 8;
const SPACING: i32 = 8;

pub trait Text {
    fn text(
        header: &Rc<Cell<Header>>,
        callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static,
    ) -> Self;
}

impl Text for gtk::Box {
    fn text(
        header: &Rc<Cell<Header>>,
        callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static,
    ) -> Self {
        // Init components
        let control = Rc::new(Control::build(header));
        let form = TextView::form();

        //header.take().dialog();

        // Init widget
        let g_box = gtk::Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .orientation(Orientation::Vertical)
            .spacing(SPACING)
            //.margin_top(MARGIN)
            .build();

        g_box.append(&form);
        g_box.append(&control.g_box);

        // Connect events

        form.buffer().connect_changed({
            let control = control.clone();
            let form = form.clone();
            move |this| control.update(this.char_count(), form.text().len())
        });

        control.upload.connect_clicked({
            let form = form.clone();
            let header = header.clone();
            move |this| {
                this.set_uploading();
                let header = header.take();
                callback(
                    Header {
                        mime: match header.mime {
                            Some(mime) => Some(mime),
                            None => Some("text/plain".into()), // server may reject the request without content type
                        },
                        token: header.token,
                    },
                    Bytes::from(form.text().as_bytes()),
                    Box::new({
                        let this = this.clone();
                        move || this.set_resend() // on failure
                    }),
                )
            }
        });

        g_box
    }
}
