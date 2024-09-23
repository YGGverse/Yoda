mod pin;
mod title;
mod widget;

use std::sync::Arc;

pub struct Label {
    // Components
    pin: Arc<pin::Pin>,
    title: Arc<title::Title>,

    // Extras
    is_pinned: bool,
    widget: widget::Label,
}

impl Label {
    // Construct
    pub fn new(is_pinned: bool) -> Arc<Label> {
        // Components
        let pin = pin::Pin::new(is_pinned);
        let title = title::Title::new();

        // Extras
        let widget = widget::Label::new(pin.widget().image(), title.widget().label());

        // Result
        Arc::new(Self {
            pin,
            title,
            is_pinned,
            widget,
        })
    }

    // Actions
    pub fn pin(&mut self) {
        self.is_pinned = !self.is_pinned; // toggle
                                          // @TODO
    }

    // Getters
    pub fn widget(&self) -> &widget::Label {
        &self.widget
    }
}
