mod widget;

use widget::Widget;

use gtk::WindowControls;
use std::rc::Rc;

pub struct Control {
    widget: Rc<Widget>,
}

impl Control {
    // Construct
    pub fn new_rc() -> Rc<Self> {
        Rc::new(Self {
            widget: Widget::new_rc(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &WindowControls {
        self.widget.gobject()
    }
}
