mod tab;
mod widget;

pub struct Main {
    widget: widget::Main,
    tab: tab::Tab,
}

impl Main {
    // Construct
    pub fn new() -> Main {
        // Init components
        let tab = tab::new();

        // Init struct
        Self {
            widget: widget::Main::new(tab.widget.as_ref()), // @TODO
            tab,
        }
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
