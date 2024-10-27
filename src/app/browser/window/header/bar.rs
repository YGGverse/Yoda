mod append;
mod control;
mod menu;
mod tab;
mod widget;

use append::Append;
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
        action_tool_debug: SimpleAction,
        action_tool_profile: SimpleAction,
        action_quit: SimpleAction,
        action_tab_append: SimpleAction,
        action_tab_close: SimpleAction,
        action_tab_close_all: SimpleAction,
        action_tab_page_navigation_base: SimpleAction,
        action_tab_page_navigation_history_back: SimpleAction,
        action_tab_page_navigation_history_forward: SimpleAction,
        action_tab_page_navigation_reload: SimpleAction,
        action_tab_pin: SimpleAction,
        view: &TabView,
    ) -> Arc<Self> {
        // Init components
        let control = Control::new_arc();
        let tab = Tab::new_arc(view);
        let append = Append::new_arc(action_tab_append.clone());
        let menu = Menu::new_arc(
            action_tool_debug,
            action_tool_profile,
            action_quit,
            action_tab_append,
            action_tab_close,
            action_tab_close_all,
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_tab_pin,
        );

        // Build result
        Arc::new(Self {
            widget: Widget::new_arc(
                control.gobject(),
                append.gobject(),
                menu.gobject(),
                tab.gobject(),
            ),
        })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
