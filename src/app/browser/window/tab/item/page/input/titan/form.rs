mod widget;

use widget::Widget;

use gtk::gio::SimpleAction;
use std::rc::Rc;

pub struct Form {
    pub widget: Rc<Widget>,
}

impl Form {
    // Constructors

    /// Build new `Self`
    pub fn build(action_update: SimpleAction) -> Self {
        Self {
            widget: Rc::new(Widget::new(action_update)),
        }
    }
}
