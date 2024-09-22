mod widget;

pub struct Forward {
    widget: widget::Forward,
}

impl Forward {
    // Construct
    pub fn new() -> Forward {
        Self {
            widget: widget::Forward::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Forward {
        &self.widget
    }
}
