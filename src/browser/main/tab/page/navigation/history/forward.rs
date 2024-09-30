use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Forward {
    widget: Button,
}

impl Forward {
    // Construct
    pub fn new(action_tab_page_navigation_history_forward: Arc<SimpleAction>) -> Self {
        Self {
            widget: Button::builder()
                .icon_name("go-next-symbolic")
                .tooltip_text("Forward")
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
