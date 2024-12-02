use gtk::{
    prelude::WidgetExt, EventControllerMotion, GestureClick, TextBuffer, TextView, WrapMode,
};

const MARGIN: i32 = 8;

pub struct Widget {
    pub text_view: TextView,
}

impl Widget {
    // Construct
    pub fn new(
        buffer: &TextBuffer,
        primary_button_controller: GestureClick,
        middle_button_controller: GestureClick,
        motion_controller: EventControllerMotion,
    ) -> Self {
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

        text_view.add_controller(primary_button_controller);
        text_view.add_controller(middle_button_controller);
        text_view.add_controller(motion_controller);

        Self { text_view }
    }
}
