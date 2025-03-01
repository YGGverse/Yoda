use adw::{
    prelude::{EntryRowExt, PreferencesRowExt},
    PasswordEntryRow,
};
use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, WidgetExt},
};

pub struct Form {
    pub password_entry_row: PasswordEntryRow,
}

impl Form {
    // Constructors

    /// Build new `Self`
    pub fn build(action_send: SimpleAction, title: Option<&str>, _max_length: Option<i32>) -> Self {
        // Init main widget
        let password_entry_row = PasswordEntryRow::builder().show_apply_button(true).build();

        if let Some(value) = title {
            password_entry_row.set_title(value);
        }

        /* @TODO adw 1.6 / ubuntu 24.10+
        if let Some(value) = max_length {
            password_entry_row.set_max_length(value);
        } */

        // Init events
        password_entry_row.connect_apply(move |_| {
            action_send.activate(None);
        });

        password_entry_row.connect_realize(|this| {
            this.grab_focus();
        });

        // Return activated struct
        Self { password_entry_row }
    }
}
