mod widget;
use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::rc::Rc;

pub struct Send {
    widget: Rc<Widget>,
}

impl Send {
    // Construct
    pub fn new(action_send: SimpleAction) -> Self {
        // Init widget
        let widget = Rc::new(Widget::new(action_send));

        // Result
        Self { widget }
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
