use gtk::Button;
use std::sync::Arc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self {
            gobject: Button::builder()
                .icon_name("starred-symbolic")
                .tooltip_text("Bookmark")
                .sensitive(false)
                .build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
