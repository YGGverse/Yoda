mod widget;

use widget::Widget;

use std::rc::Rc;

pub struct Title {
    pub widget: Rc<Widget>,
}

impl Title {
    // Construct
    pub fn new(title: Option<&str>) -> Self {
        Self {
            widget: Rc::new(Widget::new(title)),
        }
    }
}
