use super::{Profile, WindowAction};
use gtk::{
    Button, Entry,
    prelude::{ActionExt, ButtonExt, EditableExt, WidgetExt},
};
use std::{rc::Rc, sync::Arc};

const ICON_NAME: (&str, &str) = ("non-starred-symbolic", "starred-symbolic");
const TOOLTIP_TEXT: (&str, &str) = ("Add Bookmark", "Remove Bookmark");

pub struct Bookmark {
    profile: Arc<Profile>,
    request: Entry,
    pub button: Button,
}

impl Bookmark {
    pub fn build(action: &Rc<WindowAction>, profile: &Arc<Profile>, request: &Entry) -> Self {
        let button = Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.bookmark.simple_action.name()
            ))
            .build();
        update(profile, &button, request.text());
        request.connect_changed({
            let profile = profile.clone();
            let button = button.clone();
            move |entry| update(&profile, &button, entry.text())
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
        let query = self.request.text();
        let title = title.map(|t| t.to_string());
        gtk::glib::spawn_future_local(async move {
            button.set_sensitive(false); // lock
            let has_bookmark = gtk::gio::spawn_blocking(move || {
                profile.bookmark.toggle(&query, title.as_deref()).unwrap()
            })
            .await
            .unwrap();
            button.set_icon_name(icon_name(has_bookmark));
            button.set_tooltip_text(Some(tooltip_text(has_bookmark)));
            button.set_sensitive(true);
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

fn update(profile: &Arc<Profile>, button: &Button, request: gtk::glib::GString) {
    let profile = profile.clone();
    let button = button.clone();
    gtk::glib::spawn_future_local(async move {
        button.set_sensitive(false); // lock
        let has_bookmark =
            gtk::gio::spawn_blocking(move || profile.bookmark.is_match_request(&request))
                .await
                .unwrap();
        button.set_icon_name(icon_name(has_bookmark));
        button.set_tooltip_text(Some(tooltip_text(has_bookmark)));
        button.set_sensitive(true);
    }); // may take a while
}
