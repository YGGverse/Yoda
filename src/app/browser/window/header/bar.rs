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
        action_tool_debug: Arc<SimpleAction>,
        action_tool_profile: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
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
