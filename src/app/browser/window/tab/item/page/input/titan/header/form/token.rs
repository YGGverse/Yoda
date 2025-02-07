use gtk::Entry;

pub trait Token {
    fn token(text: &str) -> Self;
}

impl Token for Entry {
    fn token(text: &str) -> Self {
        const TEXT: &str = "Token";
        Entry::builder()
            .placeholder_text(TEXT)
            .text(text)
            .tooltip_text(TEXT)
            .build()
    }
}
