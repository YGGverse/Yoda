use super::WindowAction;
use gtk::{prelude::ActionExt, Button};
use std::rc::Rc;

pub trait Back {
    fn back(action: &Rc<WindowAction>) -> Self;
}

impl Back for Button {
    fn back(action: &Rc<WindowAction>) -> Self {
        Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.history_back.simple_action.name()
            )) // @TODO
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .build()
    }
}
