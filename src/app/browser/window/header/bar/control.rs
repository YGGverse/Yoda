mod widget;
use widget::Widget;

use std::rc::Rc;

pub struct Control {
    pub widget: Rc<Widget>,
}

impl Control {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Rc::new(Widget::new()),
        }
    }
}
