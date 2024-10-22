mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Back {
    action_tab_page_navigation_history_back: Arc<SimpleAction>,
    widget: Arc<Widget>,
}

impl Back {
    // Construct
    pub fn new_arc(action_tab_page_navigation_history_back: Arc<SimpleAction>) -> Arc<Self> {
        // Return activated struct
        Arc::new(Self {
            action_tab_page_navigation_history_back: action_tab_page_navigation_history_back
                .clone(),
            widget: Widget::new_arc(action_tab_page_navigation_history_back),
        })
    }

    // Actions
    pub fn update(&self, status: bool) {
        // Update actions
        self.action_tab_page_navigation_history_back
            .set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.widget.gobject()
    }
}
