mod widget;
use widget::Widget;

use crate::app::browser::window::tab::item::Action;
use std::rc::Rc;

pub struct Identity {
    action: Rc<Action>,
    pub widget: Rc<Widget>,
}

impl Identity {
    // Construct
    pub fn new(action: Rc<Action>) -> Self {
        Self {
            action: action.clone(),
            widget: Rc::new(Widget::new(action)),
        }
    }

    // Actions
    pub fn update(&self, is_auth: bool, is_enabled: bool) {
        // Update action status
        self.action.ident.simple_action.set_enabled(is_enabled);

        // Update widget
        self.widget.update(is_auth, is_enabled)
    }
}
