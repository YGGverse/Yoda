use super::{Control, Header};
use gtk::{
    gio::FileQueryInfoFlags,
    glib::{Bytes, Priority},
    Button,
};
use std::{cell::RefCell, rc::Rc};

pub struct File {
    header: Rc<RefCell<Header>>,
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
        let header = Rc::new(RefCell::new(Header {
            mime: None,
            token: None,
        }));

        let buffer = Rc::new(RefCell::new(None));

        let button = Button::builder()
            .label("Choose a file..")
            .margin_top(4)
            .build();

        // Init events
        button.connect_clicked({
            let control = control.clone();
            let buffer = buffer.clone();
            let header = header.clone();
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
                        let header = header.clone();
                        let buffer = buffer.clone();
                        let this = this.clone();
                        move |result| match result {
                            Ok(file) => match file.path() {
                                Some(path) => {
                                    this.set_label("Buffering, please wait.."); // @TODO progress
                                    file.load_bytes_async(Cancellable::NONE, {
                                        let file = file.clone();
                                        move |result| {
                                            match result {
                                                Ok((bytes, _)) => {
                                                    // try autocomplete content type (if None)
                                                    if header.borrow().mime.is_none() {
                                                        file.query_info_async(
                                                            "standard::content-type",
                                                            FileQueryInfoFlags::NONE,
                                                            Priority::DEFAULT,
                                                            Cancellable::NONE,
                                                            move |file_info| {
                                                                if let Ok(file_info) = file_info {
                                                                    header.borrow_mut().mime =
                                                                        file_info.content_type();
                                                                }
                                                                // async operations completed, unlock the form
                                                                control.update(
                                                                    Some(bytes.len()),
                                                                    None,
                                                                );
                                                                buffer.replace(Some(bytes));
                                                                this.set_css_classes(&[CLASS.2]);
                                                                this.set_label(
                                                                    path.to_str().unwrap(),
                                                                );
                                                                this.set_sensitive(true);
                                                            },
                                                        );
                                                    // no async operations left, update/unlock immediately
                                                    } else {
                                                        control.update(Some(bytes.len()), None);
                                                        buffer.replace(Some(bytes));

                                                        this.set_css_classes(&[CLASS.2]);
                                                        this.set_label(path.to_str().unwrap());
                                                        this.set_sensitive(true);
                                                    }
                                                }
                                                Err(e) => {
                                                    this.set_css_classes(&[CLASS.0]);
                                                    this.set_label(e.message());
                                                    this.set_sensitive(true);
                                                }
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

        Self {
            header,
            buffer,
            button,
        }
    }

    // Getters

    /// Get `Header` copy
    /// * borrow, do not take to have form re-send ability
    pub fn header(&self) -> Header {
        self.header.borrow().clone()
    }

    /// Get cloned [Bytes](https://docs.gtk.org/glib/struct.Bytes.html)
    // * borrow, do not take to have form re-send ability
    pub fn bytes(&self) -> Option<Bytes> {
        self.buffer.borrow().as_ref().map(|bytes| bytes.clone())
    }

    /// Get size
    pub fn size(&self) -> Option<usize> {
        self.buffer.borrow().as_ref().map(|bytes| bytes.len())
    }

    // Setters

    /// Replace current `Header`
    /// * return previous object
    pub fn set_header(&self, header: Header) -> Header {
        self.header.replace(header)
    }
}
