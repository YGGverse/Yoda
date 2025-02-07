use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};

pub trait Upload {
    fn upload() -> Self;
    fn set_uploading(&self);
    fn set_resend(&self);
}

impl Upload for Button {
    fn upload() -> Self {
        Button::builder()
            // @TODO this class not looks well with default GTK Notebook widget
            // activate it after upgrade to `ToggleGroup` in Adw v1.7 / Ubuntu 26.04
            // .css_classes(["accent"]) // | `suggested-action`
            .label("Upload")
            .sensitive(false)
            .build()
    }
    fn set_uploading(&self) {
        self.set_sensitive(false);
        self.set_label("uploading..");
    }
    fn set_resend(&self) {
        self.set_sensitive(true);
        self.set_label("Resend");
    }
}
