use super::WindowAction;
use gtk::{
    prelude::{ActionExt, ButtonExt},
    Button,
};
use std::rc::Rc;

const ICON_YES: &str = "starred-symbolic";
const ICON_NON: &str = "non-starred-symbolic";

pub trait Bookmark {
    fn bookmark(action: &Rc<WindowAction>) -> Self;
    fn _update(&self, has_bookmark: bool); // @TODO
}

impl Bookmark for Button {
    fn bookmark(action: &Rc<WindowAction>) -> Self {
        Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.bookmark.simple_action.name()
            )) // @TODO
            .icon_name(ICON_NON)
            .tooltip_text("Bookmark")
            .build()
    }

    fn _update(&self, has_bookmark: bool) {
        self.set_icon_name(if has_bookmark { ICON_YES } else { ICON_NON });
    }
}
