use gtk::{
    gdk::Paintable,
    prelude::{IsA, WidgetExt},
    ContentFit, Picture,
};

pub struct Image {
    gobject: Picture,
}

impl Image {
    // Defaults

    pub const DEFAULT_MARGIN: i32 = 6;
    pub const DEFAULT_CONTENT_FIT: ContentFit = ContentFit::ScaleDown;

    // Constructors

    pub fn new_from_paintable(paintable: &impl IsA<Paintable>) -> Self {
        let gobject = Picture::for_paintable(paintable);

        gobject.set_content_fit(Self::DEFAULT_CONTENT_FIT);
        gobject.set_margin_end(Self::DEFAULT_MARGIN);
        gobject.set_margin_start(Self::DEFAULT_MARGIN);

        Self { gobject }
    }

    // Getters

    pub fn gobject(&self) -> &Picture {
        &self.gobject
    }
}
