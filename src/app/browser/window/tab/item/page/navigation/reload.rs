mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Reload {
    action_page_reload: SimpleAction,
    widget: Arc<Widget>,
}

impl Reload {
    // Construct
    pub fn new_arc(action_page_reload: SimpleAction) -> Arc<Self> {
        Arc::new(Self {
            action_page_reload: action_page_reload.clone(),
            widget: Widget::new_arc(action_page_reload),
        })
    }

    // Actions
    pub fn update(&self, is_enabled: bool) {
        // Update actions
        self.action_page_reload.set_enabled(is_enabled);

        // Update child components
        self.widget.update(is_enabled);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
