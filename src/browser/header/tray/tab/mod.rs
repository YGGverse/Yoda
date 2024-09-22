mod widget;

pub struct Tab {
    pub widget: widget::Tab,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        Self {
            widget: widget::Tab::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Tab {
        &self.widget
    }
}
