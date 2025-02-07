use super::Header;
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::{cell::RefCell, rc::Rc};

pub trait Options {
    fn options(header: &Rc<RefCell<Header>>) -> Self;
}

impl Options for Button {
    fn options(header: &Rc<RefCell<Header>>) -> Self {
        let button = Button::builder()
            .icon_name("emblem-system-symbolic")
            .tooltip_text("Options")
            .build();

        button.connect_clicked({
            let header = header.clone();
            move |this| {
                this.set_sensitive(false); // lock
                header.take().dialog(Some(this), {
                    let this = this.clone();
                    let header = header.clone();
                    move |options| {
                        header.replace(options);
                        this.set_sensitive(true); // unlock
                    }
                })
            }
        });

        button
    }
}
