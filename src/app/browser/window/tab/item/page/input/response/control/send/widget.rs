use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new(action_send: SimpleAction) -> Self {
        // Init gobject
        let gobject = Button::builder()
            //.css_classes(["accent"])
            .label("Send")
            .build();

        // Init events
        gobject.connect_clicked({
            move |_| {
                action_send.activate(None);
            }
        });

        // Return activated `Self`
        Self { gobject }
    }

    // Actions
    pub fn update(&self, is_sensitive: bool) {
        self.gobject.set_sensitive(is_sensitive);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
