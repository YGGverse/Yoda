mod tab;
mod widget;

use std::sync::Arc;

pub struct Main {
    widget: widget::Main,
    tab: tab::Tab,
}

impl Main {
    // Construct
    pub fn new() -> Arc<Main> {
        // Init components
        let tab = tab::Tab::new();

        // Init struct
        Arc::new(Self {
            widget: widget::Main::new(tab.widget().gtk()), // @TODO
            tab,
        })
    }

    // Actions
    pub fn tab_append(&self) {
        self.tab.append(true);
    }

    // Getters
    pub fn widget(&self) -> &widget::Main {
        &self.widget
    }
}
