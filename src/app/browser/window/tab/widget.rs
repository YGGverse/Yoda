use gtk::Notebook;

pub struct Widget {
    gobject: Notebook,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        let gobject = Notebook::builder().scrollable(true).build();

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Notebook {
        &self.gobject
    }
}
