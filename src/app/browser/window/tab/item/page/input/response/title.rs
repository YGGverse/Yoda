mod widget;

use widget::Widget;

use gtk::Label;
use std::rc::Rc;

pub struct Title {
    widget: Rc<Widget>,
}

impl Title {
    // Construct
    pub fn new_rc(title: Option<&str>) -> Rc<Self> {
        // Init widget
        let widget = Widget::new_rc(title);

        // Result
        Rc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        self.widget.gobject()
    }
}
