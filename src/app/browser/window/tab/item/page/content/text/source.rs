use sourceview::{Buffer, StyleScheme, View};
const MARGIN: i32 = 8;

pub struct Source {
    pub text_view: View,
}

impl Source {
    pub fn new(data: &str) -> Self {
        Self {
            text_view: View::builder()
                .bottom_margin(MARGIN)
                .buffer(
                    &Buffer::builder()
                        .text(data)
                        .style_scheme(&StyleScheme::builder().build()) // adaptive
                        .highlight_syntax(true)
                        .build(),
                )
                .cursor_visible(false)
                .editable(false)
                .left_margin(MARGIN)
                .monospace(true)
                .right_margin(MARGIN)
                .show_line_marks(true)
                .show_line_numbers(true)
                .top_margin(MARGIN)
                .vexpand(true)
                .build(),
        }
    }
}
