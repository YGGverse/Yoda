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
    pub fn build(action_send: SimpleAction, title: Option<&str>, max_length: Option<i32>) -> Self {
        // Init widget
        let widget = Rc::new(Widget::build(action_send, title, max_length));

        // Result
        Self { widget }
    }
}
