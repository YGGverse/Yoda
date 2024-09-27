mod menu;
mod tab;

use gtk::gio::SimpleAction;
use gtk::prelude::BoxExt;
use gtk::{Box, Orientation};
use menu::Menu;
use tab::Tab;

pub struct Tray {
    widget: Box,
}

impl Tray {
    pub fn new(
        action_debug: &SimpleAction,
        action_quit: &SimpleAction,
        action_tab_append: &SimpleAction,
        action_tab_close: &SimpleAction,
        action_tab_close_all: &SimpleAction,
        action_tab_page_reload: &SimpleAction,
        action_tab_pin: &SimpleAction,
    ) -> Self {
        // Init components
        let menu = Menu::new(
            action_debug,
            action_quit,
            action_tab_append,
            action_tab_close,
            action_tab_close_all,
            action_tab_page_reload,
            action_tab_pin,
        );
        let tab = Tab::new();

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
