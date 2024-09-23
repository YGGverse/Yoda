mod tab;

use gtk::{Box, Orientation};
use tab::Tab;

use gtk::prelude::BoxExt;

pub struct Main {
    tab: Tab,
    widget: Box,
}

impl Main {
    // Construct
    pub fn new() -> Main {
        // Init components
        let tab = Tab::new();

        // Extras
        let widget = Box::builder().orientation(Orientation::Vertical).build();

        widget.append(tab.widget());

        // Init struct
        Self { tab, widget }
    }

    // Actions
    pub fn tab_append(&self) {
        self.tab.append(true);
    }

    pub fn tab_close(&self) {
        self.tab.close();
    }

    pub fn tab_close_all(&self) {
        self.tab.close();
    }

    pub fn tab_pin(&self) {
        self.tab.pin();
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
