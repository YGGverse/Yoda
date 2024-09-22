mod widget;

pub struct Back {
    widget: widget::Back,
}

impl Back {
    // Construct
    pub fn new() -> Back {
        Self {
            widget: widget::Back::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Back {
        &self.widget
    }
}
