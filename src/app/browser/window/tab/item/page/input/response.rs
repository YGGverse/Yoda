mod control;
mod form;
mod title;

use control::Control;
use form::Form;
use title::Title;

use super::ItemAction;
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri, UriHideFlags},
    prelude::BoxExt,
    Box, Label, Orientation, TextView,
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
        // Init local actions
        let action_update = SimpleAction::new(&uuid_string_random(), None);
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let control = Rc::new(Control::build(action_send.clone()));
        let form = TextView::form(action_update.clone());
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
        g_box.append(&form);
        g_box.append(&control.g_box);

        // Init events
        action_update.connect_activate({
            let base = base.clone();
            let control = control.clone();
            let form = form.clone();
            move |_, _| {
                control.update(
                    form.text().is_empty(),
                    size_limit.map(|limit| {
                        limit as isize
                            - ((base.to_string_partial(UriHideFlags::QUERY).len()
                                + Uri::escape_string(&form.text(), None, false).len())
                                as isize)
                    }),
                )
            }
        });

        action_send.connect_activate({
            let form = form.clone();
            move |_, _| {
                item_action.load.activate(
                    Some(&format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(&form.text(), None, false),
                    )),
                    false, // prevent re-send on history navigation
                );
            }
        });

        // Return activated `Self`
        g_box
    }
}
