use gtk::Entry;

const PLACEHOLDER_TEXT: &str = "Identity name (optional)";
const MARGIN: i32 = 8;

pub struct Name {
    pub gobject: Entry,
}

impl Name {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gobject: Entry::builder()
                .max_length(36) // @TODO use profile const
                .placeholder_text(PLACEHOLDER_TEXT)
                .margin_top(MARGIN)
                .build(),
        }
    }
}
