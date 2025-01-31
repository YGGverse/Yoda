use gtk::{prelude::WidgetExt, Label};
use plurify::Plurify;

pub trait Counter {
    fn counter() -> Self;
    fn update(&self, is_empty: bool, bytes_left: Option<isize>);
}

impl Counter for Label {
    fn counter() -> Self {
        Label::builder().build()
    }

    fn update(&self, is_empty: bool, bytes_left: Option<isize>) {
        match bytes_left {
            Some(value) => {
                // Update color on chars left reached
                self.set_css_classes(&[if value.is_positive() {
                    "success"
                } else {
                    "error"
                }]); // @TODO add warning step?

                // Update text
                self.set_label(&value.to_string());

                // Toggle visibility on chars left provided
                self.set_visible(!is_empty);

                self.set_tooltip_text(Some(&format!(
                    "{value} {} left",
                    (value as usize).plurify(&["byte", "bytes", "bytes"])
                )));
            }
            None => self.set_visible(false),
        }
    }
}
