use super::WindowAction;
use gtk::{prelude::ActionExt, Button};
use std::rc::Rc;

pub trait Reload {
    fn reload(action: &Rc<WindowAction>) -> Self;
}

impl Reload for Button {
    fn reload(action: &Rc<WindowAction>) -> Self {
        Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.reload.simple_action.name()
            )) // @TODO
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reload")
            .build()
    }
}
