mod widget;

use widget::Widget;

use gtk::Label;
use std::rc::Rc;

pub struct Title {
    widget: Rc<Widget>,
}

impl Title {
    // Construct
    pub fn new(title: Option<&str>) -> Self {
        Self {
            widget: Rc::new(Widget::new(title)),
        }
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        self.widget.gobject()
    }
}
