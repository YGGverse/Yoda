use gtk::{TextTag, WrapMode};

pub trait Quote {
    fn quote() -> Self;
}

impl Quote for TextTag {
    fn quote() -> Self {
        TextTag::builder()
            .left_margin(28)
            .wrap_mode(WrapMode::Word)
            .build()
    }
}
