use gtk::{pango::Style, TextTag, WrapMode};

pub fn new() -> TextTag {
    TextTag::builder()
        .style(Style::Italic)
        .wrap_mode(WrapMode::Word)
        .build()
}
