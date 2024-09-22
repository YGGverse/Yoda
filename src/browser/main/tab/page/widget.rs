use gtk::prelude::BoxExt;

pub struct Page {
    gtk: gtk::Box,
}

impl Page {
    // Construct
    pub fn new(navigation: &gtk::Box, content: &gtk::Box) -> Page {
        let gtk = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        gtk.append(navigation);
        gtk.append(content);

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
