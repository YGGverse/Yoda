mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::rc::Rc;

pub struct Append {
    pub widget: Rc<Widget>,
}

impl Append {
    // Construct
    pub fn new_rc(action_page_new: SimpleAction) -> Rc<Self> {
        Rc::new(Self {
            widget: Widget::new_rc(action_page_new),
        })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
