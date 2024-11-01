mod control;
mod menu;
mod tab;
mod widget;

use control::Control;
use menu::Menu;
use tab::Tab;
use widget::Widget;

use adw::TabView;
use gtk::{gio::SimpleAction, Box};
use std::sync::Arc;

pub struct Bar {
    widget: Arc<Widget>,
}

impl Bar {
    // Construct
    pub fn new_arc(
        action_debug: SimpleAction,
        action_profile: SimpleAction,
        action_quit: SimpleAction,
        action_page_new: SimpleAction,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_base: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_page_pin: SimpleAction,
        view: &TabView,
    ) -> Arc<Self> {
        // Init components
        let control = Control::new_arc();
        let tab = Tab::new_arc(action_page_new.clone(), view);
        let menu = Menu::new_arc(
            action_debug,
            action_profile,
            action_quit,
            action_page_new,
            action_page_close,
            action_page_close_all,
            action_page_base,
            action_page_history_back,
            action_page_history_forward,
            action_page_reload,
            action_page_pin,
        );

        // Build result
        Arc::new(Self {
            widget: Widget::new_arc(control.gobject(), menu.gobject(), tab.gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
