mod widget;

use widget::Widget;

use super::WindowAction;
use std::rc::Rc;

pub struct Back {
    action: Rc<WindowAction>,
    pub widget: Rc<Widget>,
}

impl Back {
    // Constructors

    /// Build new `Self`
    pub fn build(action: &Rc<WindowAction>) -> Self {
        Self {
            action: action.clone(),
            widget: Rc::new(Widget::build(action)),
        }
    }

    // Actions

    pub fn update(&self, status: bool) {
        // Update actions
        self.action.history_back.simple_action.set_enabled(status);

        // Update child components
        self.widget.update(status);
    }
}
