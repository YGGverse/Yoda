use gtk::{gdk_pixbuf::Pixbuf, prelude::WidgetExt, Picture};

const MARGIN: i32 = 6;

pub struct Image {
    gobject: Picture,
}

impl Image {
    // Construct
    pub fn new_from_pixbuf(buffer: &Pixbuf) -> Self {
        let gobject = Picture::for_pixbuf(buffer);

        gobject.set_margin_end(MARGIN);
        gobject.set_margin_start(MARGIN);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Picture {
        &self.gobject
    }
}
