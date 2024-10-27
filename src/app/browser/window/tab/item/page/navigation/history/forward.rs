mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Forward {
    action_tab_page_navigation_history_forward: SimpleAction,
    widget: Arc<Widget>,
}

impl Forward {
    // Construct
    pub fn new_arc(action_tab_page_navigation_history_forward: SimpleAction) -> Arc<Self> {
        // Return activated struct
        Arc::new(Self {
            action_tab_page_navigation_history_forward: action_tab_page_navigation_history_forward
                .clone(),
            widget: Widget::new_arc(action_tab_page_navigation_history_forward),
        })
    }

    // Actions
    pub fn update(&self, status: bool) {
        // Update actions
        self.action_tab_page_navigation_history_forward
            .set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.widget.gobject()
    }
}
