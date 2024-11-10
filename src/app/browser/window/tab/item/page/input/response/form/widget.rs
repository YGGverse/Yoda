use gtk::{
    gio::SimpleAction,
    glib::GString,
    prelude::{ActionExt, TextBufferExt, TextViewExt, WidgetExt},
    TextView, WrapMode,
};

const MARGIN: i32 = 8;

pub struct Widget {
    gobject: TextView,
}

impl Widget {
    // Construct
    pub fn new(action_update: SimpleAction) -> Self {
        // Init gobject
        let gobject = TextView::builder()
            .bottom_margin(MARGIN)
            .left_margin(MARGIN)
            .right_margin(MARGIN)
            .top_margin(MARGIN)
            .wrap_mode(WrapMode::Word)
            .build();

        // Init events
        gobject.buffer().connect_changed(move |_| {
            action_update.activate(None);
        });

        // Return activated `Self`
        Self { gobject }
    }

    // Actions
    pub fn focus(&self) {
        self.gobject.grab_focus();
    }

    // Getters
    pub fn text(&self) -> GString {
        let buffer = self.gobject.buffer();
        buffer.text(&buffer.start_iter(), &buffer.end_iter(), true)
    }

    pub fn gobject(&self) -> &TextView {
        &self.gobject
    }
}
