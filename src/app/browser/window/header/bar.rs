mod control;
mod menu;
mod tab;
mod widget;

use control::Control;
use menu::Menu;
use tab::Tab;
use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use adw::TabView;
use gtk::{gio::SimpleAction, Box};
use std::rc::Rc;

pub struct Bar {
    widget: Rc<Widget>,
}

impl Bar {
    // Construct
    pub fn new_rc(
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        view: &TabView,
    ) -> Rc<Self> {
        // Init components
        let control = Control::new_rc();
        let tab = Tab::new_rc(window_action.clone(), view);
        let menu = Menu::new_rc(
            browser_action,
            window_action,
            action_page_close,
            action_page_close_all,
            action_page_history_back,
            action_page_history_forward,
        );

        // Build result
        Rc::new(Self {
            widget: Widget::new_rc(control.gobject(), menu.gobject(), tab.gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        self.widget.gobject()
    }
}
