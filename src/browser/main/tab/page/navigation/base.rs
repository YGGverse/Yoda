use gtk::{glib::Uri, prelude::WidgetExt, Button};

pub struct Base {
    widget: Button,
}

impl Base {
    // Construct
    pub fn new() -> Self {
        let widget = Button::builder()
            .action_name("win.tab_page_base")
            .icon_name("go-home-symbolic")
            .tooltip_text("Base")
            .sensitive(false)
            .build();

        Self { widget }
    }

    // Actions
    pub fn update(&self, uri: Option<Uri>) {
        let status = match uri {
            Some(uri) => "/" != uri.path(),
            None => false,
        };

        self.widget.action_set_enabled("win.tab_page_base", status);
        self.widget.set_sensitive(status);
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
