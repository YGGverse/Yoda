use gtk::{Align, Label};

pub struct Title {
    pub label: Label,
}

impl Title {
    // Constructors

    /// Build new `Self`
    pub fn build(title: Option<&str>) -> Self {
        Self {
            label: Label::builder()
                .css_classes(["heading"])
                .halign(Align::Start)
                .label(title.unwrap_or("Titan input"))
                .visible(false)
                .build(),
        }
    }
}
