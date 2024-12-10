use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, SignalHandlerId},
    prelude::ActionExt,
    Align, Button,
};

// Defaults

const CSS_CLASSES: [&str; 1] = ["error"];
const LABEL: &str = "Choose";
const MARGIN: i32 = 16;

/// Choose destination [File](https://docs.gtk.org/gio/iface.File.html)
/// to write bytes on download
pub struct Choose {
    pub action: SimpleAction,
    pub button: Button,
}

impl Choose {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        let action = SimpleAction::new(&uuid_string_random(), None);

        let button = Button::builder()
            .action_name(action.name())
            .css_classes(CSS_CLASSES)
            .halign(Align::Center)
            .label(LABEL)
            .margin_top(MARGIN)
            .build();

        Self { action, button }
    }

    // Actions

    /// Formatted action connector for external implementation
    pub fn on_activate(
        &self,
        callback: impl Fn(SimpleAction, Button) + 'static,
    ) -> SignalHandlerId {
        self.action.connect_activate({
            let action = self.action.clone();
            let button = self.button.clone();
            move |_, _| callback(action.clone(), button.clone())
        })
    }
}
