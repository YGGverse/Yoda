mod widget;

pub struct Description {
    widget: widget::Description,
}

impl Description {
    // Construct
    pub fn new() -> Description {
        Self {
            widget: widget::Description::new(),
        }
    }

    // Actions
    pub fn update(&self, text: &str) {
        self.widget.update(text);
    }

    // Getters
    pub fn widget(&self) -> &widget::Description {
        &self.widget
    }
}
