mod widget;

use widget::Widget;

use std::rc::Rc;

pub struct Bookmark {
    widget: Rc<Widget>,
}

impl Bookmark {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Rc::new(Widget::new()),
        }
    }

    // Actions
    pub fn update(&self) {
        // @TODO
    }

    // Getters

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }
}
