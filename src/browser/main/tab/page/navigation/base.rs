use gtk::{glib::Uri, prelude::WidgetExt, Button};

pub struct Base {
    widget: Button,
}

impl Base {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Button::builder()
                .action_name("win.tab_page_base")
                .icon_name("go-home-symbolic")
                .tooltip_text("Base")
                .sensitive(false)
                .build(),
        }
    }

    // Actions
    pub fn update(&self, uri: Option<Uri>) {
        self.widget.set_sensitive(match uri {
            Some(uri) => "/" != uri.path(),
            None => false,
        });
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
