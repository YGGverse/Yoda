mod widget;

use std::sync::Arc;

pub struct Pin {
    widget: widget::Pin,
}

impl Pin {
    // Construct
    pub fn new() -> Arc<Pin> {
        Arc::new(Self {
            widget: widget::Pin::new(),
        })
    }

    // Getters
    pub fn widget(&self) -> &widget::Pin {
        &self.widget
    }
}
