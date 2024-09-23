mod widget;

use std::sync::Arc;

pub struct Title {
    widget: widget::Title,
}

impl Title {
    // Construct
    pub fn new() -> Arc<Title> {
        Arc::new(Self {
            widget: widget::Title::new(),
        })
    }

    // Getters
    pub fn widget(&self) -> &widget::Title {
        &self.widget
    }
}
