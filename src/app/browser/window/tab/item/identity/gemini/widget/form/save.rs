mod certificate;
use certificate::Certificate;

use super::list::{item::Value, List};
use crate::profile::Profile;
use gtk::{
    gio::{Cancellable, FileCreateFlags, ListStore},
    glib::Priority,
    prelude::{ButtonExt, FileExt, OutputStreamExtManual, WidgetExt},
    Button, FileDialog, FileFilter, Window,
};
use std::{path::MAIN_SEPARATOR, rc::Rc};

const LABEL: &str = "Export";
const TOOLTIP_TEXT: &str = "Export selected identity to file";
const MARGIN: i32 = 8;

pub struct Save {
    pub button: Button,
}

impl Save {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, list: Rc<List>) -> Self {
        // Init main widget
        let button = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .tooltip_text(TOOLTIP_TEXT)
            .visible(false)
            .build();

        // Init events
        button.connect_clicked({
            let button = button.clone();
            move |_| {
                // Get selected identity from holder
                match list.selected().value_enum() {
                    Value::ProfileIdentityGeminiId(profile_identity_gemini_id) => {
                        // Lock open button (prevent double click)
                        button.set_sensitive(false);

                        // Create PEM file based on option ID selected
                        match Certificate::new(profile.clone(), profile_identity_gemini_id) {
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
                                    .initial_name(format!(
                                        "{}.pem",
                                        certificate.name.replace(MAIN_SEPARATOR, "-")
                                    ))
                                    .build()
                                    .save(Window::NONE, Cancellable::NONE, {
                                        let button = button.clone();
                                        move |result| {
                                            match result {
                                                Ok(file) => match file.replace(
                                                    None,
                                                    false,
                                                    FileCreateFlags::NONE,
                                                    Cancellable::NONE, // @TODO
                                                ) {
                                                    Ok(stream) => stream.write_async(
                                                        certificate.data,
                                                        Priority::DEFAULT,
                                                        Cancellable::NONE, // @TODO
                                                        {
                                                            let button = button.clone();
                                                            move |result| match result {
                                                                Ok(_) => {
                                                                    button.set_css_classes(&[
                                                                        "success",
                                                                    ]);
                                                                    button.set_label(&format!(
                                                                        "Saved to {}",
                                                                        file.parse_name()
                                                                    ))
                                                                }
                                                                Err((_, e)) => {
                                                                    button.set_css_classes(&[
                                                                        "error",
                                                                    ]);
                                                                    button.set_label(&e.to_string())
                                                                }
                                                            }
                                                        },
                                                    ),
                                                    Err(e) => {
                                                        button.set_css_classes(&["error"]);
                                                        button.set_label(&e.to_string())
                                                    }
                                                },
                                                Err(e) => {
                                                    button.set_css_classes(&["warning"]);
                                                    button.set_label(e.message())
                                                }
                                            }
                                            button.set_sensitive(true); // unlock
                                        }
                                    });
                            }
                            Err(e) => {
                                button.set_css_classes(&["error"]);
                                button.set_label(&e.to_string())
                            }
                        }
                    }
                    _ => todo!(), // unexpected
                }
            }
        });

        // Return activated `Self`
        Self { button }
    }

    // Actions

    pub fn update(&self, is_visible: bool) {
        self.button.set_visible(is_visible)
    }
}
