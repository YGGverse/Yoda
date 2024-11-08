mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::rc::Rc;

pub struct Forward {
    action_page_history_forward: SimpleAction,
    widget: Rc<Widget>,
}

impl Forward {
    // Construct
    pub fn new_rc(action_page_history_forward: SimpleAction) -> Rc<Self> {
        // Return activated struct
        Rc::new(Self {
            action_page_history_forward: action_page_history_forward.clone(),
            widget: Widget::new_rc(action_page_history_forward),
        })
    }

    // Actions
    pub fn update(&self, status: bool) {
        // Update actions
        self.action_page_history_forward.set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
