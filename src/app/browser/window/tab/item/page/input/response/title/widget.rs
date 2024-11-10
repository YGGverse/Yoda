use gtk::{prelude::WidgetExt, Align, Label};

pub struct Widget {
    gobject: Label,
}

impl Widget {
    // Construct
    pub fn new(title: Option<&str>) -> Self {
        let gobject = Label::builder()
            .css_classes(["heading"])
            .halign(Align::Start)
            .margin_end(8)
            .margin_start(8)
            .visible(false)
            .build();

        if let Some(label) = title {
            if !label.is_empty() {
                gobject.set_label(label);
                gobject.set_visible(true)
            }
        }

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.gobject
    }
}
