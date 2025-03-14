use super::{Profile, WindowAction};
use gtk::{
    prelude::{ActionExt, ButtonExt, EditableExt, WidgetExt},
    Button, Entry,
};
use std::{rc::Rc, sync::Arc};

const ICON_NAME: (&str, &str) = ("non-starred-symbolic", "starred-symbolic");
const TOOLTIP_TEXT: (&str, &str) = ("Add Bookmark", "Remove Bookmark");

pub trait Bookmark {
    fn bookmark(action: &Rc<WindowAction>, profile: &Arc<Profile>, request: &Entry) -> Self;
    fn update(&self, profile: &Arc<Profile>, request: &Entry);
}

impl Bookmark for Button {
    fn bookmark(action: &Rc<WindowAction>, profile: &Arc<Profile>, request: &Entry) -> Self {
        let button = Button::builder()
            .action_name(format!(
                "{}.{}",
                action.id,
                action.bookmark.simple_action.name()
            ))
            .build();
        button.update(profile, request);

        // Setup events
        action.bookmark.simple_action.connect_activate({
            let button = button.clone();
            let profile = profile.clone();
            let request = request.clone();
            move |_, _| button.update(&profile, &request)
        });

        request.connect_changed({
            let profile = profile.clone();
            let button = button.clone();
            move |this| button.update(&profile, this)
        });

        button.connect_clicked({
            let profile = profile.clone();
            let request = request.clone();
            move |this| this.update(&profile, &request)
        });

        button
    }

    fn update(&self, profile: &Arc<Profile>, request: &Entry) {
        self.set_sensitive(false); // lock
        let this = self.clone();
        let profile = profile.clone();
        let query = request.text();
        gtk::glib::spawn_future_local(async move {
            let has_bookmark =
                gtk::gio::spawn_blocking(move || profile.bookmark.is_match_request(&query))
                    .await
                    .unwrap();
            this.set_icon_name(icon_name(has_bookmark));
            this.set_tooltip_text(Some(tooltip_text(has_bookmark)));
            this.set_sensitive(true);
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
