mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use std::rc::Rc;

pub struct Bookmark {
    pub widget: Rc<Widget>,
}

impl Bookmark {
    // Construct
    pub fn new(action: Rc<WindowAction>) -> Self {
        Self {
            widget: Rc::new(Widget::new(action.clone())),
        }
    }

    // Actions
    pub fn update(&self, has_bookmark: bool) {
        self.widget.update(has_bookmark);
    }
}
