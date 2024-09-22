mod widget;

pub struct Pin {
    widget: widget::Pin,
}

impl Pin {
    // Construct
    pub fn new() -> Pin {
        Self {
            widget: widget::Pin::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Pin {
        &self.widget
    }
}
