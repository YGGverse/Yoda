use gtk::{TextTag, WrapMode};

pub trait Plain {
    fn plain() -> Self;
}

impl Plain for TextTag {
    fn plain() -> Self {
        TextTag::builder().wrap_mode(WrapMode::Word).build()
    }
}
