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

    pub fn update(&self, current: Option<usize>, total: usize) {
        if total > 0 {
            let matches = plural(total, &["match", "matches", "matches"]);
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

// Tools

fn plural<'a>(n: usize, s: &[&'a str]) -> &'a str {
    s[if (n % 100) > 4 && (n % 100) < 20 {
        2
    } else {
        [2, 0, 1, 1, 1, 2][std::cmp::min(n % 10, 5)]
    }]
}
