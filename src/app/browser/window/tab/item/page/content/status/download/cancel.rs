use gtk::{glib::SignalHandlerId, prelude::ButtonExt, Align, Button};

// Defaults

const CSS_CLASSES: [&str; 1] = ["error"];
const LABEL: &str = "Cancel";
const MARGIN: i32 = 16;

/// Cancel download using shared [Cancellable](https://docs.gtk.org/gio/class.Cancellable.html)
pub struct Cancel {
    pub button: Button,
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
