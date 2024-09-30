mod tab;

use std::sync::Arc;

use tab::Tab;

use gtk::{gio::SimpleAction, glib::GString, prelude::BoxExt, Box, Orientation};

pub struct Main {
    tab: Arc<Tab>,
    widget: Box,
}

impl Main {
    // Construct
    pub fn new(
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Self {
        // Init components
        let tab = Arc::new(Tab::new(
            action_tab_page_navigation_base,
            action_tab_page_navigation_reload,
            action_update,
        ));
        tab.activate(tab.clone());
        tab.append(Some(GString::from("gemini://geminiprotocol.net/")), true); // demo tab @TODO replace with session restore feature

        // GTK
        let widget = Box::builder().orientation(Orientation::Vertical).build();

        widget.append(tab.widget());

        // Init struct
        Self { tab, widget }
    }

    // Actions
    pub fn tab_append(&self, tab_page_navigation_request_text: Option<GString>) {
        self.tab.append(tab_page_navigation_request_text, true);
    }

    pub fn tab_page_navigation_base(&self) {
        self.tab.page_navigation_base();
    }

    pub fn tab_page_navigation_reload(&self) {
        self.tab.page_navigation_reload();
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
    pub fn tab_page_title(&self) -> Option<GString> {
        self.tab.page_title()
    }

    pub fn tab_page_description(&self) -> Option<GString> {
        self.tab.page_description()
    }

    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
