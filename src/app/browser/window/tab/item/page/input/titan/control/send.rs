use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};

pub struct Send {
    pub button: Button,
}

impl Send {
    // Constructors

    /// Build new `Self`
    pub fn build(action_send: SimpleAction) -> Self {
        // Init main widget
        let button = Button::builder()
            .css_classes(["accent"]) // | `suggested-action`
            .label("Send")
            .sensitive(false)
            .build();

        // Init events
        button.connect_clicked({
            move |_| {
                action_send.activate(None);
            }
        });

        // Return activated `Self`
        Self { button }
    }

    // Actions
    pub fn update(&self, is_sensitive: bool) {
        self.button.set_sensitive(is_sensitive);
    }
}
