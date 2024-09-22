mod base;
mod bookmark;
mod history;
mod reload;
mod request;

mod widget;

pub struct Navigation {
    widget: widget::Navigation,
}

impl Navigation {
    pub fn new() -> Navigation {
        Self {
            widget: widget::Navigation::new(
                base::Base::new().widget().gtk(),
                history::History::new().widget().gtk(),
                reload::Reload::new().widget().gtk(),
                request::Request::new().widget().gtk(),
                bookmark::Bookmark::new().widget().gtk(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Navigation {
        &self.widget
    }
}
