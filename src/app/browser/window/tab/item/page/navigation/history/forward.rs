use super::WindowAction;
use gtk::{prelude::ActionExt, Button};
use std::rc::Rc;

pub trait Forward {
    fn forward(action: &Rc<WindowAction>) -> Self;
}

impl Forward for Button {
    fn forward(action: &Rc<WindowAction>) -> Self {
        Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.history_back.simple_action.name()
            )) // @TODO
            .icon_name("go-next-symbolic")
            .tooltip_text("Forward")
            .build()
    }
}
