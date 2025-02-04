use gtk::TextView;
use sourceview::View;

pub trait Source {
    fn source(data: &str) -> Self;
    fn into_text_view(self) -> TextView;
}

impl Source for View {
    fn source(data: &str) -> Self {
        use sourceview::{Buffer, StyleScheme};
        const MARGIN: i32 = 8;
        View::builder()
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
            .build()
    }
    fn into_text_view(self) -> TextView {
        use sourceview::prelude::Cast;
        self.upcast::<TextView>()
    }
}
