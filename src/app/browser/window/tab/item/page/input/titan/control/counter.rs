use gtk::{prelude::WidgetExt, Label};

pub trait Counter {
    fn counter() -> Self;
    fn update(&self, char_count: Option<i32>);
}

impl Counter for Label {
    // Constructors

    fn counter() -> Self {
        Label::builder().css_classes(["dim-label"]).build() // @TODO use `dimmed` in Adw 1.6,
    }

    // Actions

    fn update(&self, char_count: Option<i32>) {
        match char_count {
            Some(value) => {
                self.set_label(&value.to_string());
                self.set_visible(value > 0);
            }
            None => self.set_visible(false),
        }
    }
}
