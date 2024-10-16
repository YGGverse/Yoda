use gtk::{
    gio::SimpleAction, glib::GString, prelude::{ActionExt, TextBufferExt, TextViewExt, WidgetExt}, TextView, WrapMode
};
use std::sync::Arc;

pub struct Widget {
    gobject: TextView,
}

impl Widget {
    // Construct
    pub fn new_arc(action_update: Arc<SimpleAction>) -> Arc<Self> {
        // Init gobject
        let gobject = TextView::builder()
            .left_margin(8)
            .pixels_above_lines(8)
            .pixels_below_lines(8)
            .right_margin(8)
            .wrap_mode(WrapMode::Word)
            .build();

        // Init events
        gobject.buffer().connect_changed(move |_| {
            action_update.activate(None);
        });

        // Return activated struct
        Arc::new(Self { gobject })
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
