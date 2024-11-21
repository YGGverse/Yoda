use super::Action;
use gtk::{
    gio::{Cancellable, ListStore, TlsCertificate},
    glib::{gformat, GString},
    prelude::{ButtonExt, FileExt, TlsCertificateExt, WidgetExt},
    Button, FileDialog, FileFilter, Window,
};

use std::{cell::RefCell, rc::Rc};

const LABEL: &str = "Choose file..";
const MARGIN: i32 = 8;

pub struct File {
    pem: Rc<RefCell<Option<GString>>>,
    pub gobject: Button,
}

impl File {
    // Constructors

    /// Create new `Self`
    pub fn new(action: Rc<Action>) -> Self {
        // Init PEM
        let pem = Rc::new(RefCell::new(None));

        // Init `GObject`
        let gobject = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .visible(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let gobject = gobject.clone();
            let pem = pem.clone();
            let update = action.update.clone();
            move |_| {
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
                    .filters(&filters)
                    .default_filter(&filter_pem)
                    .build()
                    .open(None::<&Window>, None::<&Cancellable>, {
                        let gobject = gobject.clone();
                        let pem = pem.clone();
                        let update = update.clone();
                        move |result| {
                            match result {
                                Ok(file) => match file.path() {
                                    Some(path) => {
                                        let filename = path.to_str().unwrap();
                                        match TlsCertificate::from_file(&filename) {
                                            Ok(certificate) => {
                                                pem.replace(to_pem(certificate));
                                                gobject.set_css_classes(&["success"]);
                                                gobject.set_label(filename)
                                            }
                                            Err(reason) => {
                                                gobject.set_css_classes(&["error"]);
                                                gobject.set_label(reason.message())
                                            }
                                        }
                                    }
                                    None => todo!(),
                                },
                                Err(reason) => {
                                    gobject.set_css_classes(&["error"]);
                                    gobject.set_label(reason.message())
                                }
                            }
                            update.activate()
                        }
                    });
            }
        });

        // Return activated `Self`
        Self { pem, gobject }
    }

    // Actions

    /// Change visibility status
    /// * grab focus on `is_visible`
    pub fn show(&self, is_visible: bool) {
        self.gobject.set_visible(is_visible);
        if is_visible {
            self.gobject.grab_focus();
        }
    }

    // Getters

    pub fn is_valid(&self) -> bool {
        self.pem.borrow().is_some()
    }
}

// Private helpers

/// Convert [TlsCertificate](https://docs.gtk.org/gio/class.TlsCertificate.html) to GString
fn to_pem(certificate: TlsCertificate) -> Option<GString> {
    let certificate_pem = certificate.certificate_pem()?;
    let private_key_pem = certificate.private_key_pem()?;
    Some(gformat!("{certificate_pem}{private_key_pem}"))
}
