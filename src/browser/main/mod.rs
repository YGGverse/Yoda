mod tab;

use tab::Tab;

use gtk::{prelude::BoxExt, Box, Orientation};
use std::sync::Arc;

pub struct Main {
    tab: Arc<Tab>,
    widget: Box,
}

impl Main {
    // Construct
    pub fn new() -> Arc<Main> {
        // Init components
        let tab = Tab::new();

        // Extras
        let widget = Box::builder().orientation(Orientation::Vertical).build();

        widget.append(tab.widget());

        // Init struct
        Arc::new(Self { tab, widget })
    }

    // Actions
    pub fn tab_append(&self) {
        self.tab.append(true);
    }

    pub fn tab_page_reload(&self) {
        self.tab.page_reload();
    }

    pub fn tab_close(&self) {
        self.tab.close();
    }

    pub fn tab_close_all(&self) {
        self.tab.close_all();
    }

    pub fn tab_pin(&self) {
        self.tab.pin();
    }

    pub fn update(&self) {
        self.tab.update();
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
