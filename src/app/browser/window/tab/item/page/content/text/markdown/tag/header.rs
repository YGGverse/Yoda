use gtk::{TextTag, WrapMode};

pub trait Header {
    fn h1() -> Self;
    fn h2() -> Self;
    fn h3() -> Self;
}

impl Header for TextTag {
    fn h1() -> Self {
        TextTag::builder()
            .foreground("#2190a4") // @TODO optional
            .scale(1.6)
            .sentence(true)
            .weight(500)
            .wrap_mode(WrapMode::Word)
            .build()
    }
    fn h2() -> Self {
        TextTag::builder()
            .foreground("#d56199") // @TODO optional
            .scale(1.4)
            .sentence(true)
            .weight(400)
            .wrap_mode(WrapMode::Word)
            .build()
    }
    fn h3() -> Self {
        TextTag::builder()
            .foreground("#c88800") // @TODO optional
            .scale(1.2)
            .sentence(true)
            .weight(400)
            .wrap_mode(WrapMode::Word)
            .build()
    }
}
