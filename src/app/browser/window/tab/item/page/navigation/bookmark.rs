use super::{Profile, Request, WindowAction};
use gtk::{
    Button,
    prelude::{ActionExt, ButtonExt, EditableExt, WidgetExt},
};
use std::rc::Rc;

const ICON_NAME: (&str, &str) = ("non-starred-symbolic", "starred-symbolic");
const TOOLTIP_TEXT: (&str, &str) = ("Add Bookmark", "Remove Bookmark");

pub struct Bookmark {
    profile: Rc<Profile>,
    request: Rc<Request>,
    pub button: Button,
}

impl Bookmark {
    pub fn build(action: &Rc<WindowAction>, profile: &Rc<Profile>, request: &Rc<Request>) -> Self {
        let button = Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.bookmark.simple_action.name()
            ))
            .build();
        update(profile, &button, &request.entry.text());
        request.entry.connect_changed({
            let b = button.clone();
            let p = profile.clone();
            move |e| update(&p, &b, &e.text())
        });
        Self {
            profile: profile.clone(),
            request: request.clone(),
            button,
        }
    }

    pub fn toggle(&self, title: Option<&str>) {
        let button = self.button.clone();
        let profile = self.profile.clone();
        let query = self.request.entry.text();
        let title = title.map(|t| t.to_string());
        button.set_sensitive(false); // lock
        let has_bookmark = profile.bookmark.toggle(&query, title.as_deref()).unwrap();
        button.set_icon_name(icon_name(has_bookmark));
        button.set_tooltip_text(Some(tooltip_text(has_bookmark)));
        button.set_sensitive(true);
    }
}

fn tooltip_text(has_bookmark: bool) -> &'static str {
    if has_bookmark {
        TOOLTIP_TEXT.1
    } else {
        TOOLTIP_TEXT.0
    }
}

fn icon_name(has_bookmark: bool) -> &'static str {
    if has_bookmark {
        ICON_NAME.1
    } else {
        ICON_NAME.0
    }
}

fn update(profile: &Profile, button: &Button, request: &str) {
    button.set_sensitive(false); // lock
    let has_bookmark = profile.bookmark.is_match_request(request);
    button.set_icon_name(icon_name(has_bookmark));
    button.set_tooltip_text(Some(tooltip_text(has_bookmark)));
    button.set_sensitive(true);
}
