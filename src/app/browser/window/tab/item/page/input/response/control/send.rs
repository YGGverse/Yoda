use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};

pub trait Send {
    fn send(action_send: SimpleAction) -> Self;
    fn update(&self, is_sensitive: bool);
}

impl Send for Button {
    // Constructors

    /// Build new `Self`
    fn send(action_send: SimpleAction) -> Self {
        // Init main widget
        let button = Button::builder()
            .css_classes(["accent"]) // | `suggested-action`
            .label("Send")
            .sensitive(false)
            .build();

        // Init events
        button.connect_clicked({
            move |this| {
                this.set_sensitive(false);
                this.set_label("sending..");
                action_send.activate(None);
            }
        });

        // Return activated `Self`
        button
    }

    // Actions
    fn update(&self, is_sensitive: bool) {
        self.set_sensitive(is_sensitive);
    }
}
