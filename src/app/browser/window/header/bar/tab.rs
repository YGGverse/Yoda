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
    pub fn new(window_action: Rc<WindowAction>, view: &TabView) -> Self {
        Self {
            widget: Rc::new(Widget::new(view, Append::new(window_action).gobject())),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        self.widget.gobject()
    }
}
