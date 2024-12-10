use gtk::Label;

// Defaults

const LABEL: &str = "Choose location to download";
const MARGIN: i32 = 16;

/// Indicate current download state as the text
/// [Label](https://docs.gtk.org/gtk4/class.Label.html)
pub struct Status {
    pub label: Label,
}

impl Status {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            label: Label::builder().label(LABEL).margin_top(MARGIN).build(),
        }
    }
}
