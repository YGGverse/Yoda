use gtk::{TextTag, WrapMode};

pub trait List {
    fn list() -> Self;
}

impl List for TextTag {
    fn list() -> Self {
        TextTag::builder()
            .left_margin(28)
            .pixels_above_lines(4)
            .pixels_below_lines(4)
            .wrap_mode(WrapMode::Word)
            .build()
    }
}
