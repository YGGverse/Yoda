use gtk::{
    gdk::Paintable,
    prelude::{IsA, WidgetExt},
    ContentFit, Picture,
};

pub struct Image {
    pub picture: Picture,
}

impl Image {
    // Defaults

    pub const DEFAULT_MARGIN: i32 = 6;
    pub const DEFAULT_CONTENT_FIT: ContentFit = ContentFit::ScaleDown;

    // Constructors

    pub fn new_from_paintable(paintable: &impl IsA<Paintable>) -> Self {
        let picture = Picture::for_paintable(paintable);

        picture.set_content_fit(Self::DEFAULT_CONTENT_FIT);
        picture.set_margin_end(Self::DEFAULT_MARGIN);
        picture.set_margin_start(Self::DEFAULT_MARGIN);

        Self { picture }
    }
}
