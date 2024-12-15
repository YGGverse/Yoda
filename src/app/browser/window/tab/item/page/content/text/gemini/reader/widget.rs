mod find;

use find::Find;

use gtk::{
    prelude::{TextViewExt, WidgetExt},
    EventControllerMotion, GestureClick, TextBuffer, TextView, TextWindowType, WrapMode,
};

const MARGIN: i32 = 8;

pub struct Widget {
    find: Find,
    pub text_view: TextView,
}

impl Widget {
    // Constructors

    /// Create new `Self`
    pub fn new(
        buffer: &TextBuffer,
        primary_button_controller: GestureClick,
        middle_button_controller: GestureClick,
        motion_controller: EventControllerMotion,
    ) -> Self {
        // Init components
        let find = Find::new();

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

        text_view.add_controller(primary_button_controller);
        text_view.add_controller(middle_button_controller);
        text_view.add_controller(motion_controller);

        // Done
        Self { find, text_view }
    }

    // Actions

    pub fn find(&self, is_visible: bool) {
        if is_visible {
            self.text_view
                .set_gutter(TextWindowType::Bottom, Some(&self.find.g_box));
            self.find.g_box.grab_focus();
        } else {
            self.text_view
                .set_gutter(TextWindowType::Bottom, gtk::Widget::NONE);
        }
    }
}
