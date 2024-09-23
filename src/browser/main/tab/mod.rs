mod label;
mod page;
mod widget;

pub struct Tab {
    widget: widget::Tab,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        Self {
            widget: widget::Tab::new(),
        }
    }

    // Actions
    pub fn append(&self, current: bool) -> u32 {
        self.widget.append(
            label::Label::new().widget().container(),
            page::Page::new().widget().container(),
            current,
        )
    }

    // Getters
    pub fn widget(&self) -> &widget::Tab {
        &self.widget
    }
}
