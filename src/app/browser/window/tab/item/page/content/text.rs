mod gemini;
mod plain;
mod source;

use super::{ItemAction, WindowAction};
use adw::ClampScrollable;
use gemini::Gemini;
use gtk::{glib::Uri, ScrolledWindow, TextView};
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
    ) -> Self {
        // Init gemtext reader
        let gemini = Gemini::build(actions, base, gemtext).unwrap(); // @TODO handle

        // Init container widget
        let clamp_scrollable = ClampScrollable::builder()
            .child(&gemini.text_view)
            .css_classes(["view"])
            .maximum_size(800)
            .build();

        grab_focus_patch(&clamp_scrollable, &gemini.text_view);

        Self {
            text_view: gemini.text_view,
            meta: Meta {
                title: gemini.title,
            },
            scrolled_window: ScrolledWindow::builder().child(&clamp_scrollable).build(),
        }
    }

    pub fn plain(data: &str) -> Self {
        let text_view = TextView::plain(data);
        let clamp_scrollable = ClampScrollable::builder()
            .child(&text_view)
            .css_classes(["view"])
            .build();

        grab_focus_patch(&clamp_scrollable, &text_view);

        Self {
            scrolled_window: ScrolledWindow::builder().child(&clamp_scrollable).build(),
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
    use gtk::{prelude::WidgetExt, GestureClick};
    let controller = GestureClick::new();

    controller.connect_pressed({
        let text_view = text_view.clone();
        move |_, _, _, _| {
            text_view.grab_focus();
        }
    });

    clamp_scrollable.add_controller(controller);
}
