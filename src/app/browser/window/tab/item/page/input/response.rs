mod control;
mod form;
mod title;

use control::Control;
use form::Form;
use title::Title;

use super::ItemAction;
use gtk::{
    Box, Label, Orientation, TextView,
    glib::{Uri, UriHideFlags},
    prelude::{BoxExt, ButtonExt, DisplayExt, TextBufferExt, TextViewExt, WidgetExt},
};
use std::rc::Rc;

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub trait Response {
    fn response(
        item_action: Rc<ItemAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) -> Self;
}

impl Response for Box {
    // Constructors

    /// Build new `Self`
    fn response(
        item_action: Rc<ItemAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) -> Self {
        // Init components
        let control = Rc::new(Control::build());
        let text_view = TextView::form();
        let title = Label::title(title);

        // Init main widget
        let g_box = Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Vertical)
            .build();

        g_box.append(&title);
        g_box.append(&text_view);
        g_box.append(&control.g_box);

        // Init events
        text_view.buffer().connect_changed({
            let base = base.clone();
            let control = control.clone();
            let text_view = text_view.clone();
            move |_| {
                control.update(
                    text_view.text().is_empty(),
                    size_limit.map(|limit| {
                        limit as isize
                            - ((base.to_string_partial(UriHideFlags::QUERY).len()
                                + Uri::escape_string(&text_view.text(), None, false).len())
                                as isize)
                    }),
                )
            }
        });

        control.send.connect_clicked({
            let text_view = text_view.clone();
            move |this| {
                this.set_sensitive(false);
                this.set_label("sending..");
                item_action.load.activate(
                    Some(&format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(&text_view.text(), None, false),
                    )),
                    false,
                    false,
                )
            }
        });

        text_view.add_controller({
            const SHORTCUT: &str = "<Primary>Return"; // @TODO optional
            let c = gtk::ShortcutController::new();
            c.add_shortcut(
                gtk::Shortcut::builder()
                    .trigger(&gtk::ShortcutTrigger::parse_string(SHORTCUT).unwrap())
                    .action(&gtk::CallbackAction::new(move |_, _| {
                        if control.send.is_sensitive() {
                            control.send.emit_activate();
                        } else {
                            control.send.display().beep();
                        }
                        gtk::glib::Propagation::Stop
                    }))
                    .build(),
            );
            c
        });

        g_box
    }
}
