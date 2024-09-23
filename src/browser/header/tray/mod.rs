mod menu;
mod tab;

use gtk::prelude::BoxExt;
use gtk::{Box, Orientation};
use menu::Menu;
use tab::Tab;

pub struct Tray {
    widget: Box,
}

impl Tray {
    pub fn new() -> Tray {
        let menu = Menu::new();
        let tab = Tab::new();

        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        widget.append(menu.widget());
        widget.append(tab.widget());

        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
