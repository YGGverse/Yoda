mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use std::rc::Rc;

pub struct Reload {
    action: Rc<WindowAction>,
    widget: Rc<Widget>,
}

impl Reload {
    // Construct
    pub fn new(action: Rc<WindowAction>) -> Self {
        Self {
            action: action.clone(),
            widget: Rc::new(Widget::new(action)),
        }
    }

    // Actions

    pub fn update(&self, is_enabled: bool) {
        // Update actions
        self.action.reload().gobject().set_enabled(is_enabled);

        // Update child components
        self.widget.update(is_enabled);
    }

    // Getters

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }
}
