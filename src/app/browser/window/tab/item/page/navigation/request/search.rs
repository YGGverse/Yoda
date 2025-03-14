mod form;

use crate::Profile;
use adw::AlertDialog;
use adw::{
    prelude::{AlertDialogExt, AlertDialogExtManual},
    ResponseAppearance,
};
use form::{list::item::Value, list::Item, Form, Query};
use gtk::prelude::{EditableExt, WidgetExt};
use sourceview::prelude::CastNone;
use std::rc::Rc;
use std::sync::Arc;

// Response variants
const RESPONSE_APPLY: (&str, &str) = ("apply", "Apply");
const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");
// const RESPONSE_MANAGE: (&str, &str) = ("manage", "Manage");

pub trait Search {
    fn search(profile: &Arc<Profile>) -> Self;
}

impl Search for AlertDialog {
    // Constructors

    /// Create new `Self`
    fn search(profile: &Arc<Profile>) -> Self {
        // Init child container
        let form = Rc::new(Form::build(profile));

        // Init main widget
        let alert_dialog = AlertDialog::builder()
            .heading("Search")
            .body("Choose default provider")
            .close_response(RESPONSE_CANCEL.0)
            .default_response(RESPONSE_APPLY.0)
            .extra_child(&form.g_box)
            .build();

        alert_dialog.add_responses(&[
            RESPONSE_CANCEL,
            // RESPONSE_MANAGE,
            RESPONSE_APPLY,
        ]);

        // Init events

        form.list.dropdown.connect_selected_item_notify({
            let alert_dialog = alert_dialog.clone();
            let form = form.clone();
            move |_| update(&alert_dialog, &form)
        });

        form.query.connect_changed({
            let alert_dialog = alert_dialog.clone();
            let form = form.clone();
            move |_| update(&alert_dialog, &form)
        });

        alert_dialog.connect_realize({
            let form = form.clone();
            move |this| update(this, &form)
        });

        alert_dialog.connect_response(Some(RESPONSE_APPLY.0), {
            let form = form.clone();
            let profile = profile.clone();
            move |this, response| {
                // Prevent double-click action
                this.set_response_enabled(response, false);

                match form.list.selected().value_enum() {
                    Value::ProfileSearchId(profile_search_id) => {
                        if profile.search.set_default(profile_search_id).is_err() {
                            todo!() // unexpected @TODO handle
                        }
                    }
                    Value::Add => {
                        if profile
                            .search
                            .add(&form.query.uri().unwrap(), true)
                            .is_err()
                        {
                            todo!() // unexpected @TODO handle
                        }
                    }
                } // @TODO thread::spawn(|| {})
            }
        });

        // Deactivate not implemented feature @TODO
        // alert_dialog.set_response_enabled(RESPONSE_MANAGE.0, false);

        // Decorate default response preset
        alert_dialog.set_response_appearance(RESPONSE_APPLY.0, ResponseAppearance::Suggested);
        /* contrast issue with Ubuntu orange accents
        alert_dialog.set_response_appearance(RESPONSE_CANCEL.0, ResponseAppearance::Destructive); */

        // Return new activated `Self`
        alert_dialog
    }
}

fn update(alert_dialog: &AlertDialog, form: &Form) {
    match form
        .list
        .dropdown
        .selected_item()
        .and_downcast::<Item>()
        .unwrap()
        .value_enum()
    {
        Value::Add => {
            form.drop.set_visible(false);
            form.query.set_visible(true);
            if form.query.focus_child().is_none() {
                form.query.grab_focus();
            }
            form.query.remove_css_class("error");
            alert_dialog.set_response_enabled(
                RESPONSE_APPLY.0,
                if form.query.is_valid() {
                    true
                } else {
                    if !form.query.text().is_empty() {
                        form.query.add_css_class("error");
                    }
                    false
                },
            );
        }
        Value::ProfileSearchId(_) => {
            form.drop.set_visible(true);
            form.query.set_visible(false);
            alert_dialog.set_response_enabled(RESPONSE_APPLY.0, !form.list.selected().is_default());
        }
    }
}
