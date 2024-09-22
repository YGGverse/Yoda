pub struct Content {
    gtk: gtk::Box,
}

impl Content {
    // Construct new object
    pub fn new() -> Content {
        Self {
            gtk: gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
