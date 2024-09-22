use gtk::prelude::BoxExt;

pub struct Subject {
    gtk: gtk::Box,
}

impl Subject {
    pub fn new(title: &gtk::Label, description: &gtk::Label) -> Subject {
        let gtk = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        gtk.append(title);
        gtk.append(description);

        Self { gtk }
    }

    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
