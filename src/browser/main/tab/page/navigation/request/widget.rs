pub struct Request {
    gtk: gtk::Entry,
}

impl Request {
    // Construct
    pub fn new() -> Request {
        Self {
            gtk: gtk::Entry::builder()
                .placeholder_text("URL or search term...")
                .hexpand(true)
                .progress_pulse_step(0.1)
                .build(),
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Entry {
        &self.gtk
    }
}
