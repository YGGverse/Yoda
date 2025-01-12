use gtk::{prelude::WidgetExt, Label};

const MARGIN: i32 = 3;

pub struct Result {
    pub label: Label,
}

impl Default for Result {
    fn default() -> Self {
        Self::new()
    }
}

impl Result {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            label: Label::builder()
                .margin_end(MARGIN)
                .margin_start(MARGIN)
                .visible(false)
                .build(),
        }
    }

    // Actions

    pub fn update(&self, current: Option<usize>, total: usize) {
        if total > 0 {
            let matches = plurify::ns(total, &["match", "matches", "matches"]);
            match current {
                Some(position) => self
                    .label
                    .set_label(&format!("{position} of {total} {matches}")),
                None => self.label.set_label(&format!("{total} {matches}")),
            }
            self.label.remove_css_class("error");
        } else {
            self.label.set_label("Phrase not found");
            self.label.add_css_class("error");
        }
    }
}
