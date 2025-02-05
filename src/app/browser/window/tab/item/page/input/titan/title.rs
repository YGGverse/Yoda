use gtk::Label;

pub trait Title {
    fn title(label: &str) -> Self;
}

impl Title for Label {
    fn title(label: &str) -> Self {
        Label::builder()
            .css_classes(["heading"])
            .label(label)
            .build()
    }
}
