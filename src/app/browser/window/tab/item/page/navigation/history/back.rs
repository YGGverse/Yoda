mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::rc::Rc;

pub struct Back {
    action_page_history_back: SimpleAction,
    widget: Rc<Widget>,
}

impl Back {
    // Construct
    pub fn new_rc(action_page_history_back: SimpleAction) -> Rc<Self> {
        // Return activated struct
        Rc::new(Self {
            action_page_history_back: action_page_history_back.clone(),
            widget: Widget::new_rc(action_page_history_back),
        })
    }

    // Actions
    pub fn update(&self, status: bool) {
        // Update actions
        self.action_page_history_back.set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
