use super::WindowAction;
use gtk::{prelude::ButtonExt, Align, Button};
use std::rc::Rc;

pub trait Append {
    fn append(window_action: &Rc<WindowAction>) -> Self;
}

impl Append for Button {
    fn append(window_action: &Rc<WindowAction>) -> Self {
        let button = Button::builder()
            .icon_name("tab-new-symbolic")
            .css_classes(["flat"])
            .valign(Align::Center)
            .tooltip_text("New tab")
            .build();

        button.connect_clicked({
            let window_action = window_action.clone();
            move |_| window_action.append.activate_default_once()
        });

        button
    }
}
