use gtk::prelude::BoxExt;

pub struct Label {
    container: gtk::Box,
}

impl Label {
    // Construct new object
    pub fn new(pin: &gtk::Image, title: &gtk::Label) -> Label {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        container.append(pin);
        container.append(title);

        Self { container }
    }

    // Getters
    pub fn container(&self) -> &gtk::Box {
        &self.container
    }
}
