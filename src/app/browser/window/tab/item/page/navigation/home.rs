use super::WindowAction;
use gtk::{prelude::ActionExt, Button};
use std::rc::Rc;

pub trait Home {
    fn home(action: &Rc<WindowAction>) -> Self;
}

impl Home for Button {
    fn home(action: &Rc<WindowAction>) -> Self {
        Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.home.simple_action.name()
            )) // @TODO
            .icon_name("go-home-symbolic")
            .tooltip_text("Home")
            .build()
    }
}
