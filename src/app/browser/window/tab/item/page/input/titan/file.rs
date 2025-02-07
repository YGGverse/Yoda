use super::Control;
use gtk::{glib::Bytes, Button};
use std::{cell::RefCell, rc::Rc};

pub struct File {
    buffer: Rc<RefCell<Option<Bytes>>>,
    pub button: Button,
}

impl File {
    pub fn build(control: &Rc<Control>) -> Self {
        use gtk::{
            gio::Cancellable,
            prelude::{ButtonExt, FileExt, WidgetExt},
            Button, FileDialog, Window,
        };

        // Init components
        let buffer = Rc::new(RefCell::new(None));

        let button = Button::builder()
            .label("Choose a file..")
            .margin_top(4)
            .build();

        // Init events
        button.connect_clicked({
            let control = control.clone();
            let buffer = buffer.clone();
            move |this| {
                const CLASS: (&str, &str, &str) = ("error", "warning", "success");

                // reset
                control.update(None, None);
                this.set_sensitive(false);
                this.remove_css_class(CLASS.0);
                this.remove_css_class(CLASS.1);
                this.remove_css_class(CLASS.2);

                FileDialog::builder()
                    .build()
                    .open(Window::NONE, Cancellable::NONE, {
                        let control = control.clone();
                        let buffer = buffer.clone();
                        let this = this.clone();
                        move |result| match result {
                            Ok(file) => match file.path() {
                                Some(path) => {
                                    this.set_label("Buffering, please wait.."); // @TODO progress
                                    file.load_bytes_async(Cancellable::NONE, move |result| {
                                        match result {
                                            Ok((bytes, _)) => {
                                                control.update(Some(bytes.len()), None);
                                                buffer.replace(Some(bytes));

                                                this.set_css_classes(&[CLASS.2]);
                                                this.set_label(path.to_str().unwrap());
                                                this.set_sensitive(true);
                                            }
                                            Err(e) => {
                                                this.set_css_classes(&[CLASS.0]);
                                                this.set_label(e.message());
                                                this.set_sensitive(true);
                                            }
                                        }
                                    })
                                }
                                None => todo!(),
                            },
                            Err(e) => {
                                this.set_css_classes(&[CLASS.1]);
                                this.set_label(e.message());
                                this.set_sensitive(true);
                            }
                        }
                    });
            }
        });

        Self { buffer, button }
    }

    /* this method is less-expensive but not useful as user
       will not able re-upload existing form on failure @TODO

    pub fn take_bytes(&self) -> Option<Bytes> {
        self.buffer.borrow_mut().take()
    } */

    pub fn to_bytes(&self) -> Option<Bytes> {
        self.buffer.borrow().as_ref().map(|bytes| bytes.clone())
    }

    pub fn size(&self) -> Option<usize> {
        self.buffer.borrow().as_ref().map(|bytes| bytes.len())
    }
}
