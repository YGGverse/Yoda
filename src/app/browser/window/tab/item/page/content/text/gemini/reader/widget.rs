use gtk::{TextBuffer, TextView, WrapMode};
use std::sync::Arc;

pub struct Widget {
    gobject: TextView,
}

impl Widget {
    // Construct
    pub fn new_arc(buffer: &TextBuffer) -> Arc<Self> {
        Arc::new(Self {
            gobject: TextView::builder()
                .editable(false)
                .cursor_visible(false)
                .wrap_mode(WrapMode::Word)
                .vexpand(true)
                .buffer(buffer)
                .build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TextView {
        &self.gobject
    }
}
