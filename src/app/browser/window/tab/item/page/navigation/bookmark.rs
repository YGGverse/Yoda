mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use std::rc::Rc;

pub struct Bookmark {
    action: Rc<WindowAction>,
    widget: Rc<Widget>,
}

impl Bookmark {
    // Construct
    pub fn new(action: Rc<WindowAction>) -> Self {
        Self {
            widget: Rc::new(Widget::new(action.clone())),
            action,
        }
    }

    // Actions
    pub fn update(&self, is_enabled: bool) {
        // Update actions
        self.action.bookmark().gobject().set_enabled(is_enabled);

        // Update child components
        self.widget.update(is_enabled);
    }

    // Getters

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }
}
