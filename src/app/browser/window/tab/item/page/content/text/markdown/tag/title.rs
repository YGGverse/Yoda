use gtk::{TextTag, WrapMode};

pub trait Title {
    fn title() -> Self;
}

impl Title for TextTag {
    fn title() -> Self {
        TextTag::builder()
            .pixels_above_lines(4)
            .pixels_below_lines(8)
            .weight(500)
            .wrap_mode(WrapMode::None)
            .build()
    }
}
