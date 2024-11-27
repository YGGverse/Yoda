use crate::app::browser::window::action::Action as WindowAction;
use gtk::{prelude::ButtonExt, Align, Button};
use std::rc::Rc;

pub struct Widget {
    pub gobject: Button,
}

impl Widget {
    // Construct
    pub fn new(window_action: Rc<WindowAction>) -> Self {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("tab-new-symbolic")
            .css_classes(["flat"])
            .valign(Align::Center)
            .tooltip_text("New tab")
            .build();

        // Init events
        gobject.connect_clicked(move |_| window_action.append.activate_default_once());

        Self { gobject }
    }
}
