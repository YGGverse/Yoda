mod pin;
mod title;
mod widget;

use pin::Pin;
use title::Title;
use widget::Widget;

use gtk::{glib::GString, Box};
use std::sync::Arc;

pub struct Label {
    // Components
    pin: Arc<Pin>,
    title: Arc<Title>,
    // GTK
    widget: Arc<Widget>,
}

impl Label {
    // Construct
    pub fn new(name: GString, is_pinned: bool) -> Label {
        // Components
        let pin = Arc::new(Pin::new(is_pinned));
        let title = Arc::new(Title::new());

        // GTK
        let widget = Arc::new(Widget::new(name, pin.gobject(), title.gobject()));

        // Result
        Self { pin, title, widget }
    }

    // Actions
    pub fn update(&self, title: Option<&GString>) {
        self.title.update(title);
        self.widget.update(title);
    }

    // Setters
    pub fn pin(&self, is_pinned: bool) {
        self.pin.pin(is_pinned);
        self.title.pin(is_pinned);
    }

    // Getters
    pub fn is_pinned(&self) -> bool {
        self.pin.is_pinned()
    }

    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
