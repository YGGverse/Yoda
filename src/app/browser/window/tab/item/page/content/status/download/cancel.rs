use gtk::{Align, Button, glib::SignalHandlerId, prelude::ButtonExt};

// Defaults

const CSS_CLASSES: [&str; 1] = ["destructive-action"];
const LABEL: &str = "Cancel";
const MARGIN: i32 = 16;

/// Cancel download using shared [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html)
pub struct Cancel {
    pub button: Button,
}

impl Default for Cancel {
    fn default() -> Self {
        Self::new()
    }
}

impl Cancel {
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
