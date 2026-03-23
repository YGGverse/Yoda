use gtk::{TextTag, WrapMode};

pub trait Title {
    fn title() -> Self;
}

impl Title for TextTag {
    fn title() -> Self {
        TextTag::builder()
            .weight(500)
            .wrap_mode(WrapMode::None)
            .build()
    }
}
