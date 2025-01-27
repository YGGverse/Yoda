use super::{Profile, WindowAction};
use gtk::{
    prelude::{ActionExt, ButtonExt, EditableExt},
    Button, Entry,
};
use std::rc::Rc;

const ICON_YES: &str = "starred-symbolic";
const ICON_NON: &str = "non-starred-symbolic";

pub trait Bookmark {
    fn bookmark(action: &Rc<WindowAction>, profile: &Rc<Profile>, request: &Entry) -> Self;
}

impl Bookmark for Button {
    fn bookmark(action: &Rc<WindowAction>, profile: &Rc<Profile>, request: &Entry) -> Self {
        let has_bookmark = profile.bookmark.get(&request.text()).is_ok();

        let button = Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.bookmark.simple_action.name()
            )) // @TODO
            .icon_name(icon_name(has_bookmark))
            .tooltip_text("Bookmark")
            .build();

        action.bookmark.simple_action.connect_activate({
            let button = button.clone();
            let profile = profile.clone();
            let request = request.clone();
            move |_, _| {
                button.set_icon_name(icon_name(profile.bookmark.get(&request.text()).is_ok()))
            }
        }); // @TODO use local action

        button.connect_clicked(move |this| this.set_icon_name(icon_name(has_bookmark)));

        button
    }
}

fn icon_name(has_bookmark: bool) -> &'static str {
    if has_bookmark {
        ICON_YES
    } else {
        ICON_NON
    }
}
