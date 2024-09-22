mod widget;

pub struct Base {
    widget: widget::Base,
}

impl Base {
    // Construct
    pub fn new() -> Base {
        Self {
            widget: widget::Base::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Base {
        &self.widget
    }
}
