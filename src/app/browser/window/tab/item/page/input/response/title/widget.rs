use gtk::{prelude::WidgetExt, Align, Label};

const MARGIN: i32 = 6;

pub struct Widget {
    pub label: Label,
}

impl Widget {
    // Construct
    pub fn new(title: Option<&str>) -> Self {
        let label = Label::builder()
            .css_classes(["heading"])
            .halign(Align::Start)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
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
