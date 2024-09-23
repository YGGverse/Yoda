use gtk::prelude::BoxExt;

pub struct Page {
    container: gtk::Box,
}

impl Page {
    // Construct
    pub fn new(navigation: &gtk::Box, content: &gtk::Box) -> Page {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        container.append(navigation);
        container.append(content);

        Self { container }
    }

    // Getters
    pub fn container(&self) -> &gtk::Box {
        &self.container
    }
}
