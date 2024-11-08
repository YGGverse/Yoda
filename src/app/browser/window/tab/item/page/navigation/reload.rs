mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::rc::Rc;

pub struct Reload {
    action_page_reload: SimpleAction,
    widget: Rc<Widget>,
}

impl Reload {
    // Construct
    pub fn new_rc(action_page_reload: SimpleAction) -> Rc<Self> {
        Rc::new(Self {
            action_page_reload: action_page_reload.clone(),
            widget: Widget::new_rc(action_page_reload),
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
