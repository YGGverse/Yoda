mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Reload {
    action_tab_page_navigation_reload: Arc<SimpleAction>,
    widget: Arc<Widget>,
}

impl Reload {
    // Construct
    pub fn new_arc(action_tab_page_navigation_reload: Arc<SimpleAction>) -> Arc<Self> {
        Arc::new(Self {
            action_tab_page_navigation_reload: action_tab_page_navigation_reload.clone(),
            widget: Widget::new_arc(action_tab_page_navigation_reload),
        })
    }

    // Actions
    pub fn update(&self, is_enabled: bool) {
        // Update actions
        self.action_tab_page_navigation_reload
            .set_enabled(is_enabled);

        // Update child components
        self.widget.update(is_enabled);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.widget.gobject()
    }
}
