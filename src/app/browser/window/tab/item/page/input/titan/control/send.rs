use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};

pub trait Send {
    fn send() -> Self;
    fn set_sending(&self);
    fn set_resend(&self);
}

impl Send for Button {
    fn send() -> Self {
        Button::builder()
            .css_classes(["accent"]) // | `suggested-action`
            .label("Send")
            .sensitive(false)
            .build()
    }
    fn set_sending(&self) {
        self.set_sensitive(false);
        self.set_label("sending..");
    }
    fn set_resend(&self) {
        self.set_sensitive(true);
        self.set_label("Resend");
    }
}
