mod widget;

pub struct Bookmark {
    widget: widget::Bookmark,
}

impl Bookmark {
    // Construct
    pub fn new() -> Bookmark {
        Self {
            widget: widget::Bookmark::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Bookmark {
        &self.widget
    }
}
