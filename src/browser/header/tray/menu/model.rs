use gtk::gio;

pub struct Menu {
    model: gio::Menu,
}

impl Menu {
    // Construct
    pub fn new() -> Menu {
        let model_tab = gio::Menu::new();
        model_tab.append(Some("New"), Some("win.tab_append"));
        model_tab.append(Some("Pin"), Some("win.tab_pin"));

        let model_tab_close = gio::Menu::new();
        model_tab_close.append(Some("Current"), Some("win.tab_close"));
        model_tab_close.append(Some("All"), Some("win.tab_close_all"));
        model_tab.append_submenu(Some("Close"), &model_tab_close);

        let model = gio::Menu::new();
        model.append_submenu(Some("Tab"), &model_tab);
        model.append(Some("Debug"), Some("win.debug"));
        model.append(Some("Quit"), Some("win.quit"));

        Self { model }
    }

    // Getters
    pub fn model(&self) -> &gio::Menu {
        &self.model
    }
}
