mod menu;
mod tab;

use menu::Menu;
use tab::Tab;

use gtk::{
    gio::SimpleAction,
    prelude::BoxExt,
    {Box, Orientation},
};

use std::sync::Arc;

pub struct Tray {
    widget: Box,
}

impl Tray {
    pub fn new(
        action_debug: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
    ) -> Self {
        // Init components
        let tab = Tab::new(action_tab_append.clone());

        let menu = Menu::new(
            action_debug,
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

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        widget.append(menu.widget());
        widget.append(tab.widget());

        // Return new struct
        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
