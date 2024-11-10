mod bar;
mod widget;

use bar::Bar;
use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use adw::{TabView, ToolbarView};
use gtk::gio::SimpleAction;
use std::rc::Rc;

pub struct Header {
    widget: Rc<Widget>,
}

impl Header {
    // Construct
    pub fn new_rc(
        // Actions
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        // Widgets
        tab_view: &TabView,
    ) -> Rc<Self> {
        // Init components
        let bar = Bar::new_rc(
            browser_action,
            window_action,
            action_page_close,
            action_page_close_all,
            action_page_home,
            action_page_history_back,
            action_page_history_forward,
            tab_view,
        );

        // Return new struct
        Rc::new(Self {
            widget: Widget::new_rc(bar.gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        self.widget.gobject()
    }
}
