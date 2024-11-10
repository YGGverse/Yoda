use gtk::Button;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        Self {
            gobject: Button::builder()
                .icon_name("starred-symbolic")
                .tooltip_text("Bookmark")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
