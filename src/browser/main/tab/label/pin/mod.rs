mod widget;

use gtk::prelude::WidgetExt;
use std::sync::Arc;

pub struct Pin {
    is_pinned: bool,
    widget: widget::Pin,
}

impl Pin {
    // Construct
    pub fn new(is_pinned: bool) -> Arc<Pin> {
        Arc::new(Self {
            is_pinned,
            widget: widget::Pin::new(is_pinned),
        })
    }

    // Actions
    pub fn toggle(&mut self) -> bool {
        // Toggle state
        self.is_pinned = !self.widget().image().is_visible();

        // Update widget
        self.widget().image().set_visible(self.is_pinned); // @TODO delegate?

        // Return state
        self.is_pinned
    }

    // Getters
    pub fn widget(&self) -> &widget::Pin {
        &self.widget
    }
}
