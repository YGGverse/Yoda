mod find;
use std::rc::Rc;

use find::Find;

use super::WindowAction;
use gtk::{
    prelude::{ButtonExt, TextViewExt, WidgetExt},
    EventControllerMotion, GestureClick, TextBuffer, TextView, TextWindowType, WrapMode,
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
        // Init components
        let find = Rc::new(Find::new(buffer));

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

        // Connect events
        action.find.connect_activate({
            let find = find.clone();
            let text_view = text_view.clone();
            move |_| {
                text_view.set_gutter(TextWindowType::Bottom, Some(&find.g_box));
                find.input.entry.grab_focus();
            }
        });

        find.navigation.back.button.connect_clicked({
            let text_view = text_view.clone();
            let navigation = find.navigation.clone();
            move |_| {
                if let Some((mut start, _)) = navigation.back() {
                    text_view.scroll_to_iter(&mut start, 0.0, false, 0.0, 0.0);
                }
            }
        });

        find.navigation.forward.button.connect_clicked({
            let text_view = text_view.clone();
            let navigation = find.navigation.clone();
            move |_| {
                if let Some((mut start, _)) = navigation.forward() {
                    text_view.scroll_to_iter(&mut start, 0.0, false, 0.0, 0.0);
                }
            }
        });

        find.close.connect_clicked({
            let text_view = text_view.clone();
            move |_| text_view.set_gutter(TextWindowType::Bottom, gtk::Widget::NONE)
        });

        // Done
        Self { text_view }
    }
}
