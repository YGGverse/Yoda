mod certificate;
use certificate::Certificate;

use crate::profile::Profile;
use gtk::{
    gio::{Cancellable, ListStore},
    prelude::{ButtonExt, FileExt, WidgetExt},
    Button, FileDialog, FileFilter, Window,
};
use std::{cell::RefCell, fs::File, io::Write, rc::Rc};

const LABEL: &str = "Export to file..";
const MARGIN: i32 = 8;

pub struct Save {
    profile_identity_gemini_id: Rc<RefCell<Option<i64>>>,
    pub gobject: Button,
}

impl Save {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>) -> Self {
        // Init selected option holder
        let profile_identity_gemini_id = Rc::new(RefCell::new(None));

        // Init `GObject`
        let gobject = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .visible(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let profile_identity_gemini_id = profile_identity_gemini_id.clone();
            let gobject = gobject.clone();
            move |_| {
                // Get selected identity from holder
                match profile_identity_gemini_id.borrow().as_ref() {
                    Some(profile_identity_gemini_id) => {
                        // Lock open button (prevent double click)
                        gobject.set_sensitive(false);

                        // Create PEM file based on option ID selected
                        match Certificate::new(profile.clone(), *profile_identity_gemini_id) {
                            Ok(certificate) => {
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
                                    .initial_name(format!("{}.pem", certificate.name))
                                    .build()
                                    .save(None::<&Window>, None::<&Cancellable>, {
                                        let gobject = gobject.clone();
                                        move |result| {
                                            match result {
                                                Ok(file) => match file.path() {
                                                    Some(path) => match File::create(&path) {
                                                        Ok(mut destination) => {
                                                            match destination.write_all(
                                                                certificate.data.as_bytes(),
                                                            ) {
                                                                Ok(_) => {
                                                                    gobject.set_css_classes(&[
                                                                        "success",
                                                                    ]);
                                                                    gobject.set_label(&format!(
                                                                        "Saved to {}",
                                                                        path.to_string_lossy()
                                                                    ))
                                                                }
                                                                Err(e) => {
                                                                    gobject.set_css_classes(&[
                                                                        "error",
                                                                    ]);
                                                                    gobject
                                                                        .set_label(&e.to_string())
                                                                }
                                                            }
                                                        }
                                                        Err(e) => {
                                                            gobject.set_css_classes(&["error"]);
                                                            gobject.set_label(&e.to_string())
                                                        }
                                                    },
                                                    None => {
                                                        gobject.set_css_classes(&["warning"]);
                                                        gobject.set_label(
                                                            "Could not init destination path",
                                                        )
                                                    }
                                                },
                                                Err(e) => {
                                                    gobject.set_css_classes(&["warning"]);
                                                    gobject.set_label(e.message())
                                                }
                                            }
                                            gobject.set_sensitive(true); // unlock
                                        }
                                    });
                            }
                            Err(e) => {
                                gobject.set_css_classes(&["error"]);
                                gobject.set_label(&e.to_string())
                            }
                        }
                    }
                    None => todo!(), // unexpected
                }
            }
        });

        // Return activated `Self`
        Self {
            profile_identity_gemini_id,
            gobject,
        }
    }

    // Actions

    /// Update `profile_identity_gemini_id` holder,
    /// toggle visibility depending on given value
    pub fn update(&self, profile_identity_gemini_id: Option<i64>) {
        self.gobject.set_visible(match profile_identity_gemini_id {
            Some(value) => {
                self.profile_identity_gemini_id.replace(Some(value));
                true
            }
            None => {
                self.profile_identity_gemini_id.replace(None);
                false
            }
        })
    }
}
