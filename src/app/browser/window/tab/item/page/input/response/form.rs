mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, glib::GString, TextView};
use std::rc::Rc;

pub struct Form {
    widget: Rc<Widget>,
}

impl Form {
    // Construct
    pub fn new_rc(action_update: SimpleAction) -> Rc<Self> {
        // Init widget
        let widget = Widget::new_rc(action_update);

        // Result
        Rc::new(Self { widget })
    }

    // Actions
    pub fn focus(&self) {
        self.widget.focus();
    }

    // Getters
    pub fn text(&self) -> GString {
        self.widget.text()
    }

    pub fn gobject(&self) -> &TextView {
        self.widget.gobject()
    }
}
