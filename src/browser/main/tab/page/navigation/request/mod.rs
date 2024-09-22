mod widget;

pub struct Request {
    widget: widget::Request,
}

impl Request {
    // Construct
    pub fn new() -> Request {
        Self {
            widget: widget::Request::new(),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Request {
        &self.widget
    }
}
