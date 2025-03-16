use gtk::{Align, Button, glib::SignalHandlerId, prelude::ButtonExt};

// Defaults

const CSS_CLASSES: [&str; 1] = ["suggested-action"];
const LABEL: &str = "Open";
const MARGIN: i32 = 16;

/// Open [File](https://docs.gtk.org/gio/iface.File.html) on download complete
pub struct Open {
    pub button: Button,
}

impl Default for Open {
    fn default() -> Self {
        Self::new()
    }
}

impl Open {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            button: Button::builder()
                .css_classes(CSS_CLASSES)
                .halign(Align::Center)
                .label(LABEL)
                .margin_top(MARGIN)
                .visible(false)
                .build(),
        }
    }

    // Actions

    /// Formatted action connector for external implementation
    pub fn on_activate(&self, callback: impl Fn(&Button) + 'static) -> SignalHandlerId {
        self.button.connect_clicked(move |this| callback(this))
    }
}
