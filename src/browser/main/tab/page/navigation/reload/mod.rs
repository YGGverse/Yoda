mod widget;

pub struct Reload {
    widget: widget::Reload,
}

impl Reload {
    // Construct
    pub fn new() -> Reload {
        Self {
            widget: widget::Reload::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Reload {
        &self.widget
    }
}
