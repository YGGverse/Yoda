use adw::{
    prelude::{EntryRowExt, PreferencesRowExt},
    PasswordEntryRow,
};
use gtk::{
    gio::SimpleAction,
    glib::GString,
    prelude::{ActionExt, EditableExt, WidgetExt},
};

pub struct Widget {
    gobject: PasswordEntryRow,
}

impl Widget {
    // Construct
    pub fn new(action_send: SimpleAction, title: Option<&str>, max_length: Option<i32>) -> Self {
        // Init gobject
        let gobject = PasswordEntryRow::builder().show_apply_button(true).build();

        if let Some(value) = title {
            gobject.set_title(value);
        }

        if let Some(value) = max_length {
            gobject.set_max_length(value);
        }

        // Init events
        gobject.connect_apply(move |_| {
            action_send.activate(None);
        });

        // Return activated struct
        Self { gobject }
    }

    // Actions
    pub fn focus(&self) {
        self.gobject.grab_focus();
    }

    // Getters
    pub fn text(&self) -> GString {
        self.gobject.text()
    }

    pub fn gobject(&self) -> &PasswordEntryRow {
        &self.gobject
    }
}
