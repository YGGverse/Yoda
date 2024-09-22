use gtk::prelude::BoxExt;

pub struct Label {
    gtk: gtk::Box,
}

impl Label {
    // Construct new object
    pub fn new(pin: &gtk::Image, title: &gtk::Label) -> Label {
        let gtk = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        gtk.append(pin);
        gtk.append(title);

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
