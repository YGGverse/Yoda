mod tab;
mod widget;

use std::sync::Arc;

pub struct Main {
    // Components
    tab: Arc<tab::Tab>,

    // Extras
    widget: widget::Main,
}

impl Main {
    // Construct
    pub fn new() -> Arc<Main> {
        // Init components
        let tab = tab::Tab::new();

        // Extras
        let widget = widget::Main::new(tab.widget().tab());

        // Init struct
        Arc::new(Self { tab, widget })
    }

    // Actions
    pub fn tab_append(&self) {
        self.tab.append(true);
    }

    pub fn tab_pin(&self) {
        self.tab.pin();
    }

    // Getters
    pub fn widget(&self) -> &widget::Main {
        &self.widget
    }
}
