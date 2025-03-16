use gtk::{
    Align, Button,
    glib::SignalHandlerId,
    prelude::{ButtonExt, WidgetExt},
};

// Defaults

const CSS_CLASSES: [&str; 1] = ["suggested-action"];
const LABEL: (&str, &str) = ("Choose location..", "Pending..");
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
            .label(if is_activate_on_release {
                LABEL.1
            } else {
                LABEL.0
            })
            .margin_top(MARGIN)
            .sensitive(!is_activate_on_release)
            .build();

        button.connect_sensitive_notify(|this| {
            if this.is_sensitive() {
                this.set_label(LABEL.0)
            } else {
                this.set_label(LABEL.1)
            }
        });

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
