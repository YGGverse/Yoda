mod widget;
use widget::{form::list::item::value::Value, Widget};

use super::Profile;
use gtk::{glib::Uri, prelude::IsA};
use std::rc::Rc;

pub struct Default {
    // profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Default {
    // Construct

    /// Create new `Self` for given `Profile`
    pub fn build(profile: &Rc<Profile>, request: &Uri, on_apply: impl Fn() + 'static) -> Self {
        // Init widget
        let widget = Rc::new(Widget::build(profile, request));

        // Init events
        widget.on_apply({
            let profile = profile.clone();
            let request = request.clone();
            let widget = widget.clone();
            move |response| {
                // Get option match user choice
                let option = match response {
                    Value::ProfileIdentityId(value) => Some(value),
                    Value::GuestSession => None,
                    Value::GeneratePem => Some(
                        profile
                            .identity
                            .make(None, &widget.form.name.value().unwrap())
                            .unwrap(), // @TODO handle
                    ),
                    Value::ImportPem => Some(
                        profile
                            .identity
                            .add(&widget.form.file.pem.take().unwrap())
                            .unwrap(), // @TODO handle
                    ),
                };

                // Apply auth
                match option {
                    // Activate identity for `scope`
                    Some(profile_identity_id) => {
                        if profile
                            .identity
                            .auth
                            .apply(profile_identity_id, &request.to_string())
                            .is_err()
                        {
                            panic!() // unexpected @TODO
                        }
                    }
                    // Remove all identity auths for `scope`
                    None => {
                        if profile.identity.auth.remove(&request.to_string()).is_err() {
                            panic!() // unexpected @TODO
                        }
                    }
                }

                // Run callback function
                on_apply()
            }
        });

        // Return activated `Self`
        Self {
            // profile,
            widget,
        }
    }

    // Actions

    /// Show dialog for parent [Widget](https://docs.gtk.org/gtk4/class.Widget.html)
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.widget.present(parent);
    }
}
