mod gemini;
mod plain;
mod source;

use super::{ItemAction, WindowAction};
use adw::ClampScrollable;
use gemini::Gemini;
use gtk::{ScrolledWindow, TextView, glib::Uri};
use plain::Plain;
use source::Source;
use std::rc::Rc;

pub struct Meta {
    pub title: Option<String>,
} // @TODO move to separated mod

pub struct Text {
    pub meta: Meta,
    pub scrolled_window: ScrolledWindow,
    pub text_view: TextView,
}

impl Text {
    pub fn gemini(
        actions: (&Rc<WindowAction>, &Rc<ItemAction>),
        base: &Uri,
        gemtext: &str,
    ) -> Result<Self, (String, Option<Self>)> {
        match Gemini::build(actions, base, gemtext) {
            Ok(widget) => Ok(Self {
                scrolled_window: reader(&widget.text_view),
                text_view: widget.text_view,
                meta: Meta {
                    title: widget.title,
                },
            }),
            Err(e) => match e {
                gemini::Error::Markup(message, widget) => Err((
                    message,
                    Some(Self {
                        scrolled_window: reader(&widget.text_view),
                        text_view: widget.text_view,
                        meta: Meta {
                            title: widget.title,
                        },
                    }),
                )),
            },
        }
    }

    pub fn plain(data: &str) -> Self {
        let text_view = TextView::plain(data);
        Self {
            scrolled_window: reader(&text_view),
            text_view,
            meta: Meta { title: None },
        }
    }

    pub fn source(data: &str) -> Self {
        let source = sourceview::View::source(data);
        Self {
            scrolled_window: ScrolledWindow::builder().child(&source).build(),
            text_view: source.into_text_view(),
            meta: Meta { title: None },
        }
    }
}

// Tools

// Grab focus into the `TextView` on click empty `ClampScrollable` area
fn grab_focus_patch(clamp_scrollable: &ClampScrollable, text_view: &TextView) {
    use gtk::{GestureClick, prelude::WidgetExt};
    let controller = GestureClick::new();

    controller.connect_pressed({
        let text_view = text_view.clone();
        move |_, _, _, _| {
            text_view.grab_focus();
        }
    });

    clamp_scrollable.add_controller(controller);
}

fn reader(text_view: &TextView) -> ScrolledWindow {
    let clamp_scrollable = ClampScrollable::builder()
        .child(text_view)
        .css_classes(["view"])
        .maximum_size(800)
        .build();

    grab_focus_patch(&clamp_scrollable, text_view);

    ScrolledWindow::builder().child(&clamp_scrollable).build()
}
