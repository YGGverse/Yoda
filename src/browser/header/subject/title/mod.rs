mod widget;

pub struct Title {
    widget: widget::Title,
}

impl Title {
    // Construct
    pub fn new() -> Title {
        Self {
            widget: widget::Title::new(),
        }
    }

    // Actions
    pub fn update(&self, text: &str) {
        self.widget.update(text);
    }

    // Getters
    pub fn widget(&self) -> &widget::Title {
        &self.widget
    }
}
