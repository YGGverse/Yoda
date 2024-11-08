mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::rc::Rc;

pub struct Send {
    widget: Rc<Widget>,
}

impl Send {
    // Construct
    pub fn new_rc(action_send: SimpleAction) -> Rc<Self> {
        // Init widget
        let widget = Widget::new_rc(action_send);

        // Result
        Rc::new(Self { widget })
    }

    // Actions
    pub fn update(&self, is_sensitive: bool) {
        self.widget.update(is_sensitive);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
