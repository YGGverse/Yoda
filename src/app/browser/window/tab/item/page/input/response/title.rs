use gtk::{Align, Label};

pub struct Title {
    pub label: Label,
}

impl Title {
    // Constructors

    /// Build new `Self`
    pub fn build(title: Option<&str>) -> Self {
        let label = Label::builder()
            .css_classes(["heading"])
            .halign(Align::Start)
            .label(title.unwrap_or("Input expected"))
            .build();

        Self { label }
    }
}
