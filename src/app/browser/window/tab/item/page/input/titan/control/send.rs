mod widget;
use widget::Widget;

use gtk::gio::SimpleAction;
use std::rc::Rc;

pub struct Send {
    pub widget: Rc<Widget>,
}

impl Send {
    // Constructors

    /// Build new `Self`
    pub fn build(action_send: SimpleAction) -> Self {
        // Init widget
        let widget = Rc::new(Widget::build(action_send));

        // Result
        Self { widget }
    }

    // Actions
    pub fn update(&self, is_sensitive: bool) {
        self.widget.update(is_sensitive);
    }
}
