use gtk::{
    glib::{Uri, UriFlags},
    prelude::{EditableExt, EntryExt},
    Entry,
};

const MIN_LENGTH: u16 = 1;
const MAX_LENGTH: u16 = 1024;

pub trait Query {
    // Constructors

    fn query() -> Self;

    // Actions

    fn uri(&self) -> Result<Uri, String>;

    // Getters

    fn is_valid(&self) -> bool;
}

impl Query for Entry {
    // Constructors

    /// Create new `Self`
    fn query() -> Self {
        Entry::builder()
            .margin_top(8)
            .max_length(MAX_LENGTH as i32)
            .placeholder_text("Request URL (without query)")
            .visible(false)
            .build()
    }

    fn uri(&self) -> Result<Uri, String> {
        match Uri::parse(&self.text(), UriFlags::NONE) {
            Ok(uri) => {
                if !uri.scheme().is_empty()
                    && uri.host().is_some_and(|host| !host.is_empty())
                    && uri.query().is_none()
                {
                    Ok(uri)
                } else {
                    Err("Invalid query URL".to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    // Getters

    fn is_valid(&self) -> bool {
        self.text_length() >= MIN_LENGTH && self.text_length() <= MAX_LENGTH && self.uri().is_ok()
    }
}
