mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Back {
    action_page_history_back: SimpleAction,
    widget: Arc<Widget>,
}

impl Back {
    // Construct
    pub fn new_arc(action_page_history_back: SimpleAction) -> Arc<Self> {
        // Return activated struct
        Arc::new(Self {
            action_page_history_back: action_page_history_back
                .clone(),
            widget: Widget::new_arc(action_page_history_back),
        })
    }

    // Actions
    pub fn update(&self, status: bool) {
        // Update actions
        self.action_page_history_back
            .set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.widget.gobject()
    }
}
