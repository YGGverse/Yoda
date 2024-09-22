use gtk::prelude::BoxExt;

pub struct History {
    gtk: gtk::Box,
}

impl History {
    // Construct
    pub fn new(back: &gtk::Button, forward: &gtk::Button) -> History {
        let gtk = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        gtk.append(back);
        gtk.append(forward);

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
