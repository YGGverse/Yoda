use std::sync::Arc;

use gtk::{prelude::WidgetExt, Image};

pub struct Pin {
    gobject: Image,
}

impl Pin {
    // Construct
    pub fn new(visible: bool) -> Arc<Pin> {
        let gobject = Image::builder()
            .icon_name("view-pin-symbolic")
            .visible(visible)
            .build();

        Arc::new(Self { gobject })
    }

    pub fn pin(&self, is_pinned: bool) {
        self.gobject().set_visible(is_pinned);
    }

    // Getters
    pub fn gobject(&self) -> &Image {
        &self.gobject
    }
}
