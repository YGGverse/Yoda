mod widget;

use widget::Widget;

use crate::app::browser::window::Action;
use std::rc::Rc;

pub struct Back {
    pub action: Rc<Action>,
    pub widget: Rc<Widget>,
}

impl Back {
    // Constructors

    pub fn new(action: Rc<Action>) -> Self {
        Self {
            action: action.clone(),
            widget: Rc::new(Widget::new(action)),
        }
    }

    // Actions

    pub fn update(&self, status: bool) {
        // Update actions
        self.action.history_back.gobject.set_enabled(status);

        // Update child components
        self.widget.update(status);
    }
}
