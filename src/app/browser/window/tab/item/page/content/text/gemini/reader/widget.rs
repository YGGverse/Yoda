use std::rc::Rc;

use super::WindowAction;
use gtk::{
    prelude::WidgetExt, EventControllerMotion, GestureClick, TextBuffer, TextView, WrapMode,
};

const MARGIN: i32 = 8;

pub struct Widget {
    pub text_view: TextView,
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new(
        action: &WindowAction,
        buffer: &TextBuffer,
        primary_button_controller: &GestureClick,
        middle_button_controller: &GestureClick,
        motion_controller: &EventControllerMotion,
    ) -> Self {
        // Init main widget
        let text_view = TextView::builder()
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

        text_view.add_controller(primary_button_controller.clone());
        text_view.add_controller(middle_button_controller.clone());
        text_view.add_controller(motion_controller.clone());

        // Done
        Self { text_view }
    }
}
