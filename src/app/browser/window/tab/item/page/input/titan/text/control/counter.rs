use gtk::{prelude::WidgetExt, Label};
use plurify::Plurify;

pub trait Counter {
    fn counter() -> Self;
    fn update(&self, char_count: i32, bytes_total: usize);
}

impl Counter for Label {
    // Constructors

    fn counter() -> Self {
        Label::builder().css_classes(["dim-label"]).build() // @TODO use `dimmed` in Adw 1.6,
    }

    // Actions

    fn update(&self, chars_count: i32, bytes_total: usize) {
        self.set_visible(if bytes_total > 0 {
            self.set_label(&bytes_total.to_string());
            self.set_tooltip_markup(Some(&format!(
                "{bytes_total} {} <sup>/ {chars_count} {}</sup>",
                (bytes_total).plurify(&["byte", "bytes", "bytes"]),
                (chars_count as usize).plurify(&["char", "chars", "chars"]),
            )));
            true
        } else {
            false
        })
    }
}
