mod append;
mod widget;

use append::Append;
use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use adw::{TabBar, TabView};
use std::rc::Rc;

pub struct Tab {
    widget: Rc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_rc(window_action: Rc<WindowAction>, view: &TabView) -> Rc<Self> {
        Rc::new(Self {
            widget: Widget::new_rc(view, Append::new_rc(window_action).gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        self.widget.gobject()
    }
}
