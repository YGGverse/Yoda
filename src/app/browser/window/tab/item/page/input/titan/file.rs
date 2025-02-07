mod control;
mod form;

use super::Header;
use control::Control;
use gtk::Box;

pub trait File {
    fn file() -> Self;
}

impl File for Box {
    fn file() -> Self {
        use form::Form;
        use gtk::{
            gio::Cancellable,
            prelude::{ButtonExt, FileExt, WidgetExt},
            Button, FileDialog, Window,
        };
        use std::{cell::Cell, rc::Rc};

        // Init components
        let header = Rc::new(Cell::new(Header {
            mime: None,
            token: None,
        }));
        let control = Rc::new(Control::build(&header));
        let form = Button::form();

        // Init main widget
        let g_box = {
            use gtk::{prelude::BoxExt, Orientation};

            const MARGIN: i32 = 8;

            let g_box = Box::builder()
                .margin_end(MARGIN)
                .margin_start(MARGIN)
                .orientation(Orientation::Vertical)
                .spacing(MARGIN)
                .build();

            g_box.append(&form);
            g_box.append(&control.g_box);
            g_box
        };

        // Init events
        form.connect_clicked(move |form| {
            const CLASS: (&str, &str, &str) = ("error", "warning", "success");

            // reset
            control.update(None);
            form.set_sensitive(false);
            form.remove_css_class(CLASS.0);
            form.remove_css_class(CLASS.1);
            form.remove_css_class(CLASS.2);

            FileDialog::builder()
                .build()
                .open(Window::NONE, Cancellable::NONE, {
                    let control = control.clone();
                    let form = form.clone();
                    move |result| match result {
                        Ok(file) => match file.path() {
                            Some(path) => {
                                form.set_label("Buffering, please wait..");
                                file.load_bytes_async(Cancellable::NONE, move |result| match result
                                {
                                    Ok((bytes, _)) => {
                                        control.update(Some(bytes.len()));

                                        form.set_label(path.to_str().unwrap());
                                        form.set_css_classes(&[CLASS.2]);
                                        form.set_sensitive(true);
                                    }
                                    Err(e) => {
                                        form.set_css_classes(&[CLASS.0]);
                                        form.set_label(e.message());
                                        form.set_sensitive(true);
                                    }
                                })
                            }
                            None => todo!(),
                        },
                        Err(e) => {
                            form.set_css_classes(&[CLASS.1]);
                            form.set_label(e.message());
                            form.set_sensitive(true);
                        }
                    }
                });
        });

        g_box
    }
}
