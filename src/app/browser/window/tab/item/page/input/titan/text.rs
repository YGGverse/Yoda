mod control;
mod form;

use control::Control;
use control::Upload;
use form::Form;
use gtk::{
    prelude::{BoxExt, ButtonExt, TextBufferExt, TextViewExt},
    Orientation, TextView,
};
use std::rc::Rc;

const MARGIN: i32 = 8;
const SPACING: i32 = 8;

pub trait Text {
    fn text(callback: impl Fn(&[u8], Box<dyn Fn()>) + 'static) -> Self;
}

impl Text for gtk::Box {
    fn text(callback: impl Fn(&[u8], Box<dyn Fn()>) + 'static) -> Self {
        // Init components
        let control = Rc::new(Control::build());
        let form = TextView::form();

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
        control.upload.connect_clicked({
            let form = form.clone();
            move |this| {
                this.set_uploading();
                callback(
                    form.text().as_bytes(),
                    Box::new({
                        let this = this.clone();
                        move || this.set_resend() // on failure
                    }),
                )
            }
        });

        form.buffer().connect_changed({
            let control = control.clone();
            let form = form.clone();
            move |this| control.update(this.char_count(), form.text().len())
        });

        g_box
    }
}
