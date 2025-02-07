use gtk::Label;

pub trait Counter {
    fn counter() -> Self;
    fn update(&self, bytes_total: Option<usize>, chars_count: Option<i32>);
}

impl Counter for Label {
    // Constructors

    fn counter() -> Self {
        Label::builder().css_classes(["dim-label"]).build() // @TODO use `dimmed` in Adw 1.6,
    }

    // Actions

    fn update(&self, bytes_total: Option<usize>, chars_count: Option<i32>) {
        use gtk::prelude::WidgetExt;

        self.set_visible(if let Some(bytes_total) = bytes_total {
            if let Some(chars_count) = chars_count {
                if chars_count > 0 {
                    self.set_label(&bytes_total.to_string());
                    self.set_tooltip_markup(Some(&format_text_tooltip(bytes_total, chars_count)));
                    true
                } else {
                    false
                }
            } else {
                self.set_label(&format_file_tooltip(bytes_total));
                self.set_tooltip_markup(None);
                true
            }
        } else {
            false
        });
    }
}

// Tools

fn format_file_tooltip(bytes_total: usize) -> String {
    use crate::tool::Format;
    bytes_total.bytes()
}

fn format_text_tooltip(bytes_total: usize, chars_count: i32) -> String {
    use plurify::Plurify;
    format!(
        "{bytes_total} {} <sup>/ {chars_count} {}</sup>",
        (bytes_total).plurify(&["byte", "bytes", "bytes"]),
        (chars_count as usize).plurify(&["char", "chars", "chars"]),
    )
}
