use gtk::{prelude::WidgetExt, Label};

const MARGIN: i32 = 3;

pub struct Result {
    pub label: Label,
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

    pub fn show(&self, current: usize, total: usize) {
        if total > 0 {
            self.label
                .set_label(&format!("{current} if {total} matches"));
            self.label.remove_css_class("error");
        } else {
            self.label.set_label(&format!("Phrase not found"));
            self.label.add_css_class("error");
        }
        self.label.set_visible(true);
    }

    pub fn hide(&self) {
        self.label.set_visible(false);
    }
}
