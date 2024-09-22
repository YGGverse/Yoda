mod content;
mod navigation;
mod widget;

pub struct Page {
    widget: widget::Page,
}

impl Page {
    pub fn new() -> Page {
        Self {
            widget: widget::Page::new(
                navigation::Navigation::new().widget().gtk(),
                content::Content::new().widget().gtk(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Page {
        &self.widget
    }
}
