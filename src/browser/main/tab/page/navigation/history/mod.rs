mod back;
mod forward;
mod widget;

pub struct History {
    widget: widget::History,
}

impl History {
    // Construct
    pub fn new() -> History {
        Self {
            widget: widget::History::new(
                back::Back::new().widget().gtk(),
                forward::Forward::new().widget().gtk(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::History {
        &self.widget
    }
}
