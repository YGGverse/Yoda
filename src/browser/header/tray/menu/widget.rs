use gtk::gio;

pub struct Menu {
    gtk: gtk::MenuButton,
}

impl Menu {
    // Construct
    pub fn new(model: &gio::Menu) -> Menu {
        let gtk = gtk::MenuButton::builder().tooltip_text("Menu").build();

        gtk.set_menu_model(Some(model));

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::MenuButton {
        &self.gtk
    }
}
