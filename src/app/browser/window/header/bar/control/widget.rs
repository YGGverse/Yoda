use gtk::{PackType, WindowControls};

pub struct Widget {
    gobject: WindowControls,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        Self {
            gobject: WindowControls::builder()
                .side(PackType::End)
                .margin_end(4)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &WindowControls {
        &self.gobject
    }
}
