mod widget;

use widget::Widget;

use std::rc::Rc;

pub struct Counter {
    pub widget: Rc<Widget>,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Rc::new(Widget::new()),
        }
    }

    // Actions
    pub fn update(&self, bytes: Option<usize>) {
        self.widget.update(bytes);
    }
}
