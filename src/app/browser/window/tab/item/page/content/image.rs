use gtk::{gdk_pixbuf::Pixbuf, prelude::WidgetExt, Picture};

pub struct Image {
    gobject: Picture,
}

impl Image {
    // Defaults

    const DEFAULT_MARGIN: i32 = 6;

    // Constructors

    pub fn new_from_pixbuf(buffer: &Pixbuf) -> Self {
        let gobject = Picture::for_pixbuf(buffer);

        gobject.set_margin_end(Self::DEFAULT_MARGIN);
        gobject.set_margin_start(Self::DEFAULT_MARGIN);

        Self { gobject }
    }

    // Getters

    pub fn gobject(&self) -> &Picture {
        &self.gobject
    }
}
