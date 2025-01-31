mod control;
mod form;
mod title;

use control::Control;
use control::Send;
use form::Form;
use gtk::{
    prelude::{BoxExt, ButtonExt, TextBufferExt, TextViewExt},
    Label, Orientation, TextView,
};
use std::rc::Rc;
use title::Title;

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub trait Titan {
    fn titan(callback: impl Fn(&[u8], Box<dyn Fn()>) + 'static) -> Self;
}

impl Titan for gtk::Box {
    fn titan(callback: impl Fn(&[u8], Box<dyn Fn()>) + 'static) -> Self {
        // Init components
        let control = Rc::new(Control::build());
        let form = TextView::form();
        let title = Label::title(None);

        // Init widget
        let g_box = gtk::Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Vertical)
            .build();

        g_box.append(&title);
        g_box.append(&form);
        g_box.append(&control.g_box);

        // Connect events
        control.send.connect_clicked({
            let form = form.clone();
            move |this| {
                this.set_sending();
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
            move |this| control.update(Some(this.char_count()))
        });

        // Return activated `Self`
        g_box
    }
}
