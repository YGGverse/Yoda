use gtk::prelude::BoxExt;

pub struct Main {
    gtk: gtk::Box,
}

impl Main {
    // Construct
    pub fn new(tab: &gtk::Notebook) -> Main {
        let gtk = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        gtk.append(tab);

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
