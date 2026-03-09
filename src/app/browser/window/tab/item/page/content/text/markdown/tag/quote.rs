use gtk::{TextTag, WrapMode::Word, pango::Style::Italic};

pub trait Quote {
    fn quote() -> Self;
}

impl Quote for TextTag {
    fn quote() -> Self {
        TextTag::builder()
            .left_margin(28)
            .wrap_mode(Word)
            .style(Italic) // what about the italic tags decoration? @TODO
            .build()
    }
}
