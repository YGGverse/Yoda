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
        update(profile, &button, request.entry.text());
        request.entry.connect_changed({
            let b = button.clone();
            let p = profile.clone();
            move |e| update(&p, &b, e.text())
        });
        Self {
            profile: profile.clone(),
            request: request.clone(),
            button,
        }
    }

    pub fn toggle(&self, title: Option<&str>) {
        let t = title.map(|t| t.to_string());
        let p = self.profile.clone();
        let b = self.button.clone();
        let e = self.request.entry.clone();
        gtk::glib::spawn_future_local(async move {
            b.set_sensitive(false); // lock
            let has_bookmark = p.bookmark.toggle(&e.text(), t.as_deref()).unwrap();
            b.set_icon_name(icon_name(has_bookmark));
            b.set_tooltip_text(Some(tooltip_text(has_bookmark)));
            b.set_sensitive(true)
        }); // may take a while
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

fn update(profile: &Rc<Profile>, button: &Button, request: gtk::glib::GString) {
    let p = profile.clone();
    let b = button.clone();
    gtk::glib::spawn_future_local(async move {
        b.set_sensitive(false); // lock
        let has_bookmark = p.bookmark.is_match_request(&request);
        b.set_icon_name(icon_name(has_bookmark));
        b.set_tooltip_text(Some(tooltip_text(has_bookmark)));
        b.set_sensitive(true);
    }); // may take a while
}
