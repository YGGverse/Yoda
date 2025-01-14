mod widget;

use widget::Widget;

use std::rc::Rc;

pub struct Title {
    pub widget: Rc<Widget>,
}

impl Title {
    // Constructors

    /// Build new `Self`
    pub fn build(title: Option<&str>) -> Self {
        Self {
            widget: Rc::new(Widget::build(title)),
        }
    }
}
