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

    // Getters
    pub fn widget(&self) -> &widget::Title {
        &self.widget
    }
}
