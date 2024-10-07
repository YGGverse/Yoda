use gtk::{prelude::WidgetExt, Image};

pub struct Pin {
    gobject: Image,
}

impl Pin {
    // Construct
    pub fn new(visible: bool) -> Pin {
        let gobject = Image::builder()
            .icon_name("view-pin-symbolic")
            .visible(visible)
            .build();

        Self { gobject }
    }

    pub fn pin(&self, is_pinned: bool) {
        self.gobject().set_visible(is_pinned);
    }

    // Getters
    pub fn is_pinned(&self) -> bool {
        self.gobject.is_visible()
    }

    pub fn gobject(&self) -> &Image {
        &self.gobject
    }
}
