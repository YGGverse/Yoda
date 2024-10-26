use gtk::{gdk_pixbuf::Pixbuf, Picture};

pub struct Image {
    gobject: Picture,
}

impl Image {
    // Construct
    pub fn new_from_pixbuf(buffer: &Pixbuf) -> Self {
        Self {
            gobject: Picture::for_pixbuf(buffer),
        }
    }

    // Getters
    pub fn gobject(&self) -> &Picture {
        &self.gobject
    }
}
