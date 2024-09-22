mod widget;

pub struct Content {
    widget: widget::Content,
}

impl Content {
    // Construct
    pub fn new() -> Content {
        Self {
            widget: widget::Content::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Content {
        &self.widget
    }
}
