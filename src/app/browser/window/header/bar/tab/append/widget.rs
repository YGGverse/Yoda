use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt},
    Align, Button,
};
use std::sync::Arc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new_arc(action_page_new: SimpleAction) -> Arc<Self> {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("tab-new-symbolic")
            .css_classes(["flat"])
            .valign(Align::Center)
            .tooltip_text("New tab")
            .build();

        // Init events
        gobject.connect_clicked(move |_| {
            action_page_new.activate(None);
        });

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
