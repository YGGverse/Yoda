use gtk::{prelude::WidgetExt, Label};
use std::sync::Arc;

pub struct Widget {
    gobject: Label,
}

impl Widget {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        let gobject = Label::builder().use_markup(true).build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn update(&self, count: &i32, count_limit: Option<&i32>) {
        match count_limit {
            Some(limit) => {
                // Update color on limit reached
                self.gobject
                    .set_css_classes(&[if count < limit { "success" } else { "error" }]); // @TODO add warning step?

                // Update text
                self.gobject
                    .set_markup(&format!("{count} <sup>/ {limit}</sup>"));

                // Toggle visibility if limit provided
                self.gobject.set_visible(true);
            }
            None => self.gobject.set_visible(false),
        }
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.gobject
    }
}
