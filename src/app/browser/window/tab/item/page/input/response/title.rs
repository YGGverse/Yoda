use gtk::{Align, Label};

pub trait Title {
    fn title(title: Option<&str>) -> Self;
}

impl Title for Label {
    fn title(title: Option<&str>) -> Self {
        Label::builder()
            .css_classes(["heading"])
            .halign(Align::Start)
            .label(title.unwrap_or("Input expected"))
            .build()
    }
}
