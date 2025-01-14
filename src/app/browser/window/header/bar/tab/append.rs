mod widget;

use widget::Widget;

use super::WindowAction;
use std::rc::Rc;

pub struct Append {
    pub widget: Rc<Widget>,
}

impl Append {
    // Constructors

    /// Build new `Self`
    pub fn build(window_action: &Rc<WindowAction>) -> Self {
        Self {
            widget: Rc::new(Widget::build(window_action)),
        }
    }
}
