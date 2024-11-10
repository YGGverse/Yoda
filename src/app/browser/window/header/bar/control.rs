mod widget;

use widget::Widget;

use gtk::WindowControls;
use std::rc::Rc;

pub struct Control {
    widget: Rc<Widget>,
}

impl Control {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Rc::new(Widget::new()),
        }
    }

    // Getters
    pub fn gobject(&self) -> &WindowControls {
        self.widget.gobject()
    }
}
