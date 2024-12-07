use super::Action;
use gtk::{
    gio::{Cancellable, ListStore, TlsCertificate},
    glib::{gformat, GString},
    prelude::{ButtonExt, FileExt, TlsCertificateExt, WidgetExt},
    Button, FileDialog, FileFilter, Window,
};

use std::{cell::RefCell, rc::Rc};

const LABEL: &str = "Choose file..";
const TOOLTIP_TEXT: &str = "Import existing identity from file";
const MARGIN: i32 = 8;

pub struct File {
    pub pem: Rc<RefCell<Option<GString>>>,
    pub button: Button,
}

impl File {
    // Constructors

    /// Create new `Self`
    pub fn new(action_widget: Rc<Action>) -> Self {
        // Init PEM
        let pem = Rc::new(RefCell::new(None));

        // Init main gobject
        let button = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .tooltip_text(TOOLTIP_TEXT)
            .visible(false)
            .build();

        // Init events
        button.connect_clicked({
            let action_widget = action_widget.clone();
            let button = button.clone();
            let pem = pem.clone();
            move |_| {
                // Lock open button (prevent double click)
                button.set_sensitive(false);

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
                        let action_widget = action_widget.clone();
                        let button = button.clone();
                        let pem = pem.clone();
                        move |result| {
                            match result {
                                Ok(file) => match file.path() {
                                    Some(path) => {
                                        let filename = path.to_str().unwrap();
                                        match TlsCertificate::from_file(filename) {
                                            Ok(certificate) => {
                                                pem.replace(to_pem(certificate));
                                                button.set_css_classes(&["success"]);
                                                button.set_label(filename)
                                            }
                                            Err(reason) => {
                                                button.set_css_classes(&["error"]);
                                                button.set_label(reason.message())
                                            }
                                        }
                                    }
                                    None => todo!(),
                                },
                                Err(reason) => {
                                    button.set_css_classes(&["warning"]);
                                    button.set_label(reason.message())
                                }
                            }
                            button.set_sensitive(true); // unlock
                            action_widget.update.activate(true)
                        }
                    });
            }
        });

        // Return activated `Self`
        Self { pem, button }
    }

    // Actions

    /// Change visibility status
    pub fn set_visible(&self, is_visible: bool) {
        self.button.set_visible(is_visible);
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
