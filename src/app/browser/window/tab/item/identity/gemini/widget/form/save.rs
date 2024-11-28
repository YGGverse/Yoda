mod certificate;
use certificate::Certificate;

use super::Action;
use gtk::{
    gio::{Cancellable, ListStore},
    prelude::{ButtonExt, FileExt, WidgetExt},
    Button, FileDialog, FileFilter, Window,
};
use std::{cell::RefCell, fs::File, io::Write, rc::Rc};

const LABEL: &str = "Export to file..";
const MARGIN: i32 = 8;

pub struct Save {
    certificate: Rc<RefCell<Option<Certificate>>>,
    pub gobject: Button,
}

impl Save {
    // Constructors

    /// Create new `Self`
    pub fn new(action: Rc<Action>) -> Self {
        // Init `Certificate` holder
        let certificate = Rc::new(RefCell::new(None::<Certificate>));

        // Init `GObject`
        let gobject = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .visible(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let certificate = certificate.clone();
            let gobject = gobject.clone();
            move |_| {
                // Get certificate selected from holder
                match certificate.borrow().as_ref() {
                    Some(certificate) => {
                        // Copy certificate holder values
                        let name = certificate.name.clone();
                        let data = certificate.data.clone();

                        // Lock open button (prevent double click)
                        gobject.set_sensitive(false);

                        // Init file filters related with PEM extension
                        let filters = ListStore::new::<FileFilter>();

                        let filter_all = FileFilter::new();
                        filter_all.add_pattern("*.*");
                        filter_all.set_name(Some("All"));
                        filters.append(&filter_all);

                        let filter_pem = FileFilter::new();
                        filter_pem.add_mime_type("application/x-x509-ca-cert");
                        filter_pem.set_name(Some("Certificate (*.pem)"));
                        filters.append(&filter_pem);

                        // Init file dialog
                        FileDialog::builder()
                            .default_filter(&filter_pem)
                            .filters(&filters)
                            .initial_name(format!("{name}.pem"))
                            .build()
                            .save(None::<&Window>, None::<&Cancellable>, {
                                let gobject = gobject.clone();
                                move |result| {
                                    match result {
                                        Ok(file) => match file.path() {
                                            Some(path) => match File::create(&path) {
                                                Ok(mut destination) => {
                                                    match destination.write_all(data.as_bytes()) {
                                                        Ok(_) => {
                                                            // @TODO
                                                            gobject.set_css_classes(&["success"]);
                                                            gobject.set_label(&format!(
                                                                "Saved to {}",
                                                                path.to_string_lossy()
                                                            ))
                                                        }
                                                        Err(reason) => {
                                                            gobject.set_css_classes(&["error"]);
                                                            gobject.set_label(&reason.to_string())
                                                        }
                                                    }
                                                }
                                                Err(reason) => {
                                                    gobject.set_css_classes(&["error"]);
                                                    gobject.set_label(&reason.to_string())
                                                }
                                            },
                                            None => todo!(),
                                        },
                                        Err(reason) => {
                                            gobject.set_css_classes(&["warning"]);
                                            gobject.set_label(reason.message())
                                        }
                                    }
                                    gobject.set_sensitive(true); // unlock
                                }
                            });
                    }
                    None => todo!(), // unexpected
                }
            }
        });

        // Return activated `Self`
        Self {
            certificate,
            gobject,
        }
    }

    // Actions

    /// Change visibility status
    /// * grab focus on `is_visible`
    pub fn show(&self, is_visible: bool) {
        self.gobject.set_visible(is_visible)
    }
}
