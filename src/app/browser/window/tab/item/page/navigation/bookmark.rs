mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use std::rc::Rc;

pub struct Bookmark {
    pub widget: Rc<Widget>,
}

impl Bookmark {
    // Constructors

    /// Build new `Self`
    pub fn build(action: &Rc<WindowAction>) -> Self {
        Self {
            widget: Rc::new(Widget::build(action)),
        }
    }

    // Actions
    pub fn update(&self, has_bookmark: bool) {
        self.widget.update(has_bookmark);
    }
}
