mod bar;
mod widget;

use bar::Bar;
use widget::Widget;

use super::{Action as WindowAction, BrowserAction};
use adw::TabView;
use std::rc::Rc;

pub struct Header {
    pub widget: Rc<Widget>,
}

impl Header {
    // Construct
    pub fn new(
        // Actions
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        // Widgets
        tab_view: &TabView,
    ) -> Self {
        // Init components
        let bar = Bar::new(browser_action, window_action, tab_view);

        // Return new struct
        Self {
            widget: Rc::new(Widget::new(&bar.widget.g_box)),
        }
    }
}
