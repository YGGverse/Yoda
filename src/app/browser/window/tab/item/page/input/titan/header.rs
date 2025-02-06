mod form;

use gtk::{glib::GString, prelude::IsA, Widget};

#[derive(Default)]
pub struct Header {
    pub mime: Option<GString>,
    pub token: Option<GString>,
}

impl Header {
    /// Show header options dialog for the referrer `widget`
    /// * takes ownership of `Self`, return new updated copy in `callback` function
    pub fn dialog(self, widget: Option<&impl IsA<Widget>>, callback: impl Fn(Self) + 'static) {
        use adw::{
            prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
            AlertDialog, ResponseAppearance,
        };
        use form::Form;
        use std::rc::Rc;

        // Response variants
        const RESPONSE_APPLY: (&str, &str) = ("apply", "Apply");
        const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");

        // Init form components
        let form = Rc::new(Form::build(
            &self.mime.unwrap_or_default(),
            &self.token.unwrap_or_default(),
        ));

        // Init main widget
        let alert_dialog = AlertDialog::builder()
            .heading("Header")
            .body("Custom header options")
            .close_response(RESPONSE_CANCEL.0)
            .default_response(RESPONSE_APPLY.0)
            .extra_child(&form.g_box)
            .build();

        alert_dialog.add_responses(&[RESPONSE_CANCEL, RESPONSE_APPLY]);

        // Decorate default response preset
        alert_dialog.set_response_appearance(RESPONSE_APPLY.0, ResponseAppearance::Suggested);
        /* contrast issue with Ubuntu orange accents
        alert_dialog.set_response_appearance(RESPONSE_CANCEL.0, ResponseAppearance::Destructive); */

        // Init events

        alert_dialog.connect_response(None, {
            let form = form.clone();
            move |this, response| {
                this.set_response_enabled(response, false); // prevent double-click
                if response == RESPONSE_APPLY.0 {
                    callback(Self {
                        mime: form.mime(),
                        token: form.token(),
                    })
                } else {
                    // @TODO restore
                }
            }
        });

        // Show
        alert_dialog.present(widget);
    }
}
