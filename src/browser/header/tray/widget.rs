use gtk::prelude::BoxExt;

pub struct Tray {
    gtk: gtk::Box,
}

impl Tray {
    // Construct
    pub fn new(menu: &gtk::MenuButton, tab: &gtk::Button) -> Tray {
        let gtk = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();

        gtk.append(menu);
        gtk.append(tab);

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
