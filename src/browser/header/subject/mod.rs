mod description;
mod title;
mod widget;

pub struct Subject {
    widget: widget::Subject,
}

impl Subject {
    // Construct
    pub fn new() -> Subject {
        Self {
            widget: widget::Subject::new(
                title::Title::new().widget().gtk(),
                description::Description::new().widget().gtk(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Subject {
        &self.widget
    }
}
