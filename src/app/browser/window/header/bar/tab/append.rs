mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::Button;
use std::rc::Rc;

pub struct Append {
    pub widget: Rc<Widget>,
}

impl Append {
    // Construct
    pub fn new_rc(window_action: Rc<WindowAction>) -> Rc<Self> {
        Rc::new(Self {
            widget: Widget::new_rc(window_action),
        })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
