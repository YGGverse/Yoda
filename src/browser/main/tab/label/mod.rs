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
                pin::Pin::new().widget().image(),
                title::Title::new().widget().label(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Label {
        &self.widget
    }
}
