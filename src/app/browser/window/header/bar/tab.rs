mod append;
mod widget;

use append::Append;
use widget::Widget;

use super::WindowAction;
use adw::TabView;
use std::rc::Rc;

pub struct Tab {
    pub widget: Rc<Widget>,
}

impl Tab {
    // Construct
    pub fn new(window_action: &Rc<WindowAction>, view: &TabView) -> Self {
        Self {
            widget: Rc::new(Widget::build(
                view,
                &Append::build(window_action).widget.button,
            )),
        }
    }
}
