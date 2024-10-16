use gtk::{prelude::WidgetExt, TextView, WrapMode};
use std::sync::Arc;

pub struct Widget {
    gobject: TextView,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = TextView::builder()
            .left_margin(8)
            .pixels_above_lines(8)
            .pixels_below_lines(8)
            .right_margin(8)
            .wrap_mode(WrapMode::Word)
            .build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn grab_focus(&self) {
        self.gobject.grab_focus();
    }

    // Getters
    pub fn gobject(&self) -> &TextView {
        &self.gobject
    }
}
