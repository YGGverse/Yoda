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
use std::rc::Rc;

pub struct Bar {
    pub widget: Rc<Widget>,
}

impl Bar {
    // Constructors

    pub fn new(
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        view: &TabView,
    ) -> Self {
        let control = Control::new();
        let tab = Tab::new(window_action.clone(), view);
        let menu = Menu::new(browser_action, window_action);
        Self {
            widget: Rc::new(Widget::new(
                &control.widget.gobject,
                &menu.widget.gobject,
                &tab.widget.tab_bar,
            )),
        }
    }
}
