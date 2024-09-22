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
    pub fn set_text(&self, text: &str) {
        self.widget.gtk().set_text(text);
    }

    pub fn update(&self) {
        self.widget.update();
    }

    // Getters
    pub fn widget(&self) -> &widget::Description {
        &self.widget
    }
}
