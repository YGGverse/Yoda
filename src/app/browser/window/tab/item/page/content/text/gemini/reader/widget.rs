use gtk::{
    prelude::WidgetExt, EventControllerMotion, GestureClick, TextBuffer, TextView, WrapMode,
};
use std::sync::Arc;

const MARGIN: i32 = 16;

pub struct Widget {
    gobject: TextView,
}

impl Widget {
    // Construct
    pub fn new_arc(
        buffer: &TextBuffer,
        primary_button_controller: GestureClick,
        middle_button_controller: GestureClick,
        motion_controller: EventControllerMotion,
    ) -> Arc<Self> {
        let gobject = TextView::builder()
            .bottom_margin(MARGIN)
            .buffer(buffer)
            .cursor_visible(false)
            .editable(false)
            .left_margin(MARGIN)
            .right_margin(MARGIN)
            .top_margin(MARGIN)
            .vexpand(true)
            .wrap_mode(WrapMode::Word)
            .build();

        gobject.add_controller(primary_button_controller);
        gobject.add_controller(middle_button_controller);
        gobject.add_controller(motion_controller);

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &TextView {
        &self.gobject
    }
}
