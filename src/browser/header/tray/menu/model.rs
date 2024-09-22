use gtk::gio;

pub struct Menu {
    model: gio::Menu,
}

impl Menu {
    // Construct
    pub fn new() -> Menu {
        let model = gio::Menu::new();
        let model_tab = gio::Menu::new();

        model_tab.append(Some("Append"), Some("win.tab_append"));
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
