use gtk::{PackType, WindowControls};
use std::sync::Arc;

pub struct Widget {
    gobject: WindowControls,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self {
            gobject: WindowControls::new(PackType::End),
        })
    }

    // Getters
    pub fn gobject(&self) -> &WindowControls {
        &self.gobject
    }
}
