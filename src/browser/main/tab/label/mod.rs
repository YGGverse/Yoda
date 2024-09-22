mod pin;
mod title;
mod widget;

pub struct Label {
    widget: widget::Label,
}

impl Label {
    // Construct
    pub fn new() -> Label {
        Self {
            widget: widget::Label::new(
                pin::Pin::new().widget().gtk(),
                title::Title::new().widget().gtk(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Label {
        &self.widget
    }
}
