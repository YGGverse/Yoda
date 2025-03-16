mod form;

use super::ItemAction;
use form::Form;
use gtk::{
    Box, Orientation,
    gio::SimpleAction,
    glib::{Uri, UriHideFlags, uuid_string_random},
    prelude::{BoxExt, EditableExt, WidgetExt},
};
use std::rc::Rc;

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub trait Sensitive {
    fn sensitive(
        item_action: Rc<ItemAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) -> Self;
}

impl Sensitive for Box {
    // Constructors

    /// Build new `Self`
    fn sensitive(
        item_action: Rc<ItemAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) -> Self {
        // Init local actions
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let form = Rc::new(Form::build(
            action_send.clone(),
            title,
            max_length
                .map(|value| value - base.to_string_partial(UriHideFlags::QUERY).len() as i32),
        ));

        // Init widget
        let g_box = Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Vertical)
            .build();

        g_box.append(&form.password_entry_row);

        // Init events
        action_send.connect_activate({
            let form = form.clone();
            move |_, _| {
                item_action.load.activate(
                    Some(&format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(&form.password_entry_row.text(), None, false),
                    )),
                    false,
                )
            }
        });

        g_box.connect_realize(move |_| {
            form.password_entry_row.grab_focus();
        });

        // Return activated `Self`
        g_box
    }
}
