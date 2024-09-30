use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Back {
    widget: Button,
}

impl Back {
    // Construct
    pub fn new(action_tab_page_navigation_history_back: Arc<SimpleAction>) -> Self {
        Self {
            widget: Button::builder()
                .icon_name("go-previous-symbolic")
                .tooltip_text("Back")
                .sensitive(false)
                .build(),
        }
    }

    // Actions
    pub fn update(&self) {
        // @TODO
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
