mod widget;

use widget::Widget;

use gtk::Button;
use std::rc::Rc;

pub struct Bookmark {
    widget: Rc<Widget>,
}

impl Bookmark {
    // Construct
    pub fn new_rc() -> Rc<Self> {
        Rc::new(Self {
            widget: Widget::new_rc(),
        })
    }

    // Actions
    pub fn update(&self) {
        // @TODO
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
