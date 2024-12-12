use gtk::{
    glib::SignalHandlerId,
    prelude::{ButtonExt, WidgetExt},
    Align, Button,
};

// Defaults

const CSS_CLASSES: [&str; 1] = ["suggested-action"];
const LABEL: &str = "Choose location..";
const MARGIN: i32 = 16;

/// Choose destination [File](https://docs.gtk.org/gio/iface.File.html)
/// to write bytes on download
pub struct Choose {
    pub button: Button,
}

impl Choose {
    // Constructors

    /// Create new `Self`
    pub fn new(is_activate_on_release: bool) -> Self {
        let button = Button::builder()
            .css_classes(CSS_CLASSES)
            .halign(Align::Center)
            .label(LABEL)
            .margin_top(MARGIN)
            .build();

        if is_activate_on_release {
            button.connect_realize(|this| {
                this.activate();
            });
        }

        Self { button }
    }

    // Actions

    /// Formatted action connector for external implementation
    pub fn on_activate(&self, callback: impl Fn(&Button) + 'static) -> SignalHandlerId {
        self.button.connect_clicked(move |this| callback(this))
    }
}
