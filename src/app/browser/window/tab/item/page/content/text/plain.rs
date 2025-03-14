use gtk::TextView;

pub trait Plain {
    fn plain(data: &str) -> Self;
}

impl Plain for TextView {
    fn plain(data: &str) -> Self {
        const MARGIN: i32 = 8;
        TextView::builder()
            .bottom_margin(MARGIN)
            .cursor_visible(false)
            .buffer(&gtk::TextBuffer::builder().text(data).build())
            .editable(false)
            .left_margin(MARGIN)
            .monospace(true)
            .right_margin(MARGIN)
            .top_margin(MARGIN)
            .vexpand(true)
            .wrap_mode(gtk::WrapMode::Word)
            .build()
    }
}
