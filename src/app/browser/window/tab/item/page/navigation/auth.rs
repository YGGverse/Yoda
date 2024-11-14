mod widget;
use widget::Widget;

use crate::app::browser::window::tab::item::Action;
use std::rc::Rc;

pub struct Auth {
    widget: Rc<Widget>,
}

impl Auth {
    // Construct
    pub fn new(action: Rc<Action>) -> Self {
        Self {
            widget: Rc::new(Widget::new(action.clone())),
        }
    }

    // Actions
    pub fn update(&self) {
        // @TODO
    }

    // Getters

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }
}
