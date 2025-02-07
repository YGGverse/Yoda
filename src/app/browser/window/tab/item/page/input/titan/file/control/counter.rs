use gtk::{prelude::WidgetExt, Label};
use plurify::Plurify;

pub trait Counter {
    fn counter() -> Self;
    fn update(&self, bytes_total: usize);
}

impl Counter for Label {
    // Constructors

    fn counter() -> Self {
        Label::builder().css_classes(["dim-label"]).build() // @TODO use `dimmed` in Adw 1.6,
    }

    // Actions

    fn update(&self, bytes_total: usize) {
        self.set_visible(if bytes_total > 0 {
            let format = bytes_total.plurify(&["byte", "bytes", "bytes"]);
            self.set_label(format);
            self.set_tooltip_text(Some(format));
            true
        } else {
            false
        })
    }
}
