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
    pub fn append(&self, is_active: bool) -> u32 {
        self.widget.append(
            label::Label::new(false).widget().container(),
            page::Page::new().widget().container(),
            is_active,
        )
    }

    pub fn pin(&self) -> bool {
        false // @TODO
    }

    // Getters
    pub fn widget(&self) -> &widget::Tab {
        &self.widget
    }
}
