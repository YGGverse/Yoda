mod widget;

use widget::Widget;

use std::rc::Rc;

pub struct Counter {
    pub widget: Rc<Widget>,
}

impl Counter {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Rc::new(Widget::new()),
        }
    }

    // Actions
    pub fn update(&self, chars_left: Option<i32>) {
        self.widget.update(chars_left);
    }
}
