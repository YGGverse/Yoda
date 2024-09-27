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
    pub fn new(action_debug: &SimpleAction, action_quit: &SimpleAction) -> Self {
        // Init components
        let menu = Menu::new(action_debug, action_quit);
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
