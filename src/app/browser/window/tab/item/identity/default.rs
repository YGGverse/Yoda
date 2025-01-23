mod widget;
use widget::{form::list::item::value::Value, Widget};

use super::{BrowserAction, Profile, WindowAction};
use gtk::{glib::Uri, prelude::IsA};
use std::rc::Rc;

pub struct Default {
    // profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Default {
    // Construct

    /// Create new `Self` for given `Profile`
    pub fn build(
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
        profile: &Rc<Profile>,
        request: &Uri,
    ) -> Self {
        // Init widget
        let widget = Rc::new(Widget::build(
            (browser_action, window_action),
            profile,
            request,
        ));

        // Init events
        widget.on_cancel({
            let browser_action = browser_action.clone();
            move || browser_action.update.activate(None)
        });

        widget.on_apply({
            let profile = profile.clone();
            let request = request.clone();
            let widget = widget.clone();
            let window_action = window_action.clone();
            move |response| {
                // Get option match user choice
                let option = match response {
                    Value::ProfileIdentityId(value) => Some(value),
                    Value::GuestSession => None,
                    Value::GeneratePem => Some(
                        match profile
                            .identity
                            .make(None, &widget.form.name.value().unwrap())
                        {
                            Ok(profile_identity_id) => profile_identity_id,
                            Err(e) => todo!("{e}"),
                        },
                    ),
                    Value::ImportPem => Some(
                        match profile.identity.add(&widget.form.file.pem.take().unwrap()) {
                            Ok(profile_identity_id) => profile_identity_id,
                            Err(e) => todo!("{e}"),
                        },
                    ),
                };

                // Apply auth
                match option {
                    // Activate identity for `scope`
                    Some(profile_identity_id) => {
                        if let Err(e) = profile
                            .identity
                            .auth
                            .apply(profile_identity_id, &request.to_string())
                        {
                            todo!("{e}")
                        };
                    }
                    // Remove all identity auths for `scope`
                    None => {
                        if let Err(e) = profile.identity.auth.remove_scope(&request.to_string()) {
                            todo!("{e}")
                        };
                    }
                }

                // Reload page to apply changes
                window_action.reload.activate();
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
