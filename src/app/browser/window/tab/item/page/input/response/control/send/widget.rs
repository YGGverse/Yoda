use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};

const MARGIN: i32 = 6;

pub struct Widget {
    pub button: Button,
}

impl Widget {
    // Construct
    pub fn new(action_send: SimpleAction) -> Self {
        // Init main widget
        let button = Button::builder()
            //.css_classes(["accent"])
            .label("Send")
            .margin_bottom(MARGIN)
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
