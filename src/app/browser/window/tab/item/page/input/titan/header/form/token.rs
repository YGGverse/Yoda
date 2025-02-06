use gtk::Entry;

pub trait Token {
    fn token(text: &str) -> Self;
}

impl Token for Entry {
    fn token(text: &str) -> Self {
        Entry::builder()
            .placeholder_text("Token")
            .text(text)
            .build()
    }
}
