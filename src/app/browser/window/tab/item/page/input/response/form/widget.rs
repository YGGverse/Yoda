use gtk::{
    gio::SimpleAction,
    glib::GString,
    prelude::{ActionExt, TextBufferExt, TextViewExt},
    TextView, WrapMode,
};

const MARGIN: i32 = 8;

pub struct Widget {
    pub text_view: TextView,
}

impl Widget {
    // Construct
    pub fn new(action_update: SimpleAction) -> Self {
        // Init main widget
        let text_view = TextView::builder()
            .bottom_margin(MARGIN)
            .left_margin(MARGIN)
            .right_margin(MARGIN)
            .top_margin(MARGIN)
            .wrap_mode(WrapMode::Word)
            .build();

        // Init events
        text_view.buffer().connect_changed(move |_| {
            action_update.activate(None);
        });

        // Return activated `Self`
        Self { text_view }
    }

    // Getters

    pub fn text(&self) -> GString {
        let buffer = self.text_view.buffer();
        buffer.text(&buffer.start_iter(), &buffer.end_iter(), true)
    }
}
