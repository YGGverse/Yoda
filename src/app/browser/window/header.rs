mod bar;
mod widget;

use bar::Bar;
use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use adw::{TabView, ToolbarView};
use std::rc::Rc;

pub struct Header {
    widget: Rc<Widget>,
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
            widget: Rc::new(Widget::new(bar.gobject())),
        }
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        self.widget.gobject()
    }
}
