use gtk::Picture;

pub struct Image {
    gobject: Picture,
}

impl Image {
    // Construct
    pub fn new() -> Self {
        Self {
            gobject: Picture::new(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &Picture {
        &self.gobject
    }
}
