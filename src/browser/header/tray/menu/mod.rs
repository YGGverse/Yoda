mod model;
mod widget;

pub struct Menu {
    widget: widget::Menu,
}

impl Menu {
    pub fn new() -> Menu {
        Self {
            widget: widget::Menu::new(model::Menu::new().model()),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Menu {
        &self.widget
    }
}
