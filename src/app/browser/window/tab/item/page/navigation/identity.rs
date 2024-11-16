mod widget;
use widget::Widget;

use crate::app::browser::window::tab::item::Action;
use std::rc::Rc;

pub struct Identity {
    widget: Rc<Widget>,
}

impl Identity {
    // Construct
    pub fn new(action: Rc<Action>) -> Self {
        Self {
            widget: Rc::new(Widget::new(action.clone())),
        }
    }

    // Actions
    pub fn update(&self) {
        self.widget.update(false) // @TODO
    }

    // Getters

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }
}
