use gtk::{prelude::WidgetExt, Align, Label};

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
            .visible(false)
            .build();

        if let Some(value) = title {
            if !value.is_empty() {
                label.set_label(value);
                label.set_visible(true)
            }
        }

        Self { label }
    }
}
