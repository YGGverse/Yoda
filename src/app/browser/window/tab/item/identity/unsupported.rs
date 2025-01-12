mod widget;
use widget::Widget;

use gtk::prelude::IsA;
use std::rc::Rc;

pub struct Unsupported {
    widget: Rc<Widget>,
}

impl Default for Unsupported {
    fn default() -> Self {
        Self::new()
    }
}

impl Unsupported {
    // Construct

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            widget: Rc::new(Widget::new()),
        }
    }

    // Actions

    /// Show dialog for given parent
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.widget.present(parent)
    }
}
