use gtk::{PackType, WindowControls};
use std::rc::Rc;

pub struct Widget {
    gobject: WindowControls,
}

impl Widget {
    // Construct
    pub fn new_rc() -> Rc<Self> {
        Rc::new(Self {
            gobject: WindowControls::builder()
                .side(PackType::End)
                .margin_end(4)
                .build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &WindowControls {
        &self.gobject
    }
}
