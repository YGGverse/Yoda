use gtk::{prelude::WidgetExt, Label};

pub trait Counter {
    fn counter() -> Self;
    fn update(&self, bytes_total: Option<usize>);
}

impl Counter for Label {
    // Constructors

    fn counter() -> Self {
        Label::builder().css_classes(["dim-label"]).build() // @TODO use `dimmed` in Adw 1.6,
    }

    // Actions

    fn update(&self, bytes_total: Option<usize>) {
        self.set_visible(if let Some(bytes_total) = bytes_total {
            use crate::tool::Format;
            self.set_tooltip_text(Some(&bytes_total.bytes()));
            true
        } else {
            false
        })
    }
}
