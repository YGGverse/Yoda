use gtk::{TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .pixels_above_lines(4)
        .pixels_below_lines(8)
        .weight(500)
        .wrap_mode(WrapMode::None)
        .build()
}
