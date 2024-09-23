mod pin;
mod title;
mod widget;

use std::sync::Arc;

pub struct Label {
    // Components
    pin: Arc<pin::Pin>,
    title: Arc<title::Title>,

    // Extras
    widget: widget::Label,
}

impl Label {
    // Construct
    pub fn new() -> Arc<Label> {
        // Init components
        let pin = pin::Pin::new();
        let title = title::Title::new();

        // Init extras
        let widget = widget::Label::new(pin.widget().image(), title.widget().label());

        // Result
        Arc::new(Self { pin, title, widget })
    }

    // Getters
    pub fn widget(&self) -> &widget::Label {
        &self.widget
    }
}
