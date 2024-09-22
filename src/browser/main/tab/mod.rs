mod label;
mod page;
mod widget;

pub struct Tab {
    widget: widget::Tab,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        let widget = widget::Tab::new();

        Self { widget }
    }

    // Actions
    pub fn append(&self, current: bool) -> u32 {
        self.widget.append(
            label::Label::new().widget().gtk(),
            page::Page::new().widget().gtk(),
            true,
        )
    }

    // Getters
    pub fn widget(&self) -> &widget::Tab {
        &self.widget
    }
}
