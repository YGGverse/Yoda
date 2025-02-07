use gtk::Label;

pub trait Tab {
    fn tab(label: &str) -> Self;
}

impl Tab for Label {
    fn tab(label: &str) -> Self {
        Label::builder()
            .css_classes(["heading"])
            .label(label)
            .build()
    }
}
