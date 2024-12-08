use gtk::{TextBuffer, TextView};

const MARGIN: i32 = 8;

pub struct Source {
    pub text_view: TextView,
}

impl Source {
    pub fn new(data: &str) -> Self {
        Self {
            text_view: TextView::builder()
                .bottom_margin(MARGIN)
                .buffer(&TextBuffer::builder().text(data).build())
                .cursor_visible(false)
                .editable(false)
                .left_margin(MARGIN)
                .monospace(true)
                .right_margin(MARGIN)
                .top_margin(MARGIN)
                .vexpand(true)
                .build(),
        }
    }
}
