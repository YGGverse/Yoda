mod control;
mod form;
mod title;

use control::Control;
use form::Form;
use title::Title;

use super::TabAction;
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri, UriHideFlags},
    prelude::BoxExt,
    Box, Orientation,
};
use std::rc::Rc;

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub struct Response {
    // Components
    pub g_box: Box,
}

impl Response {
    // Constructors

    /// Build new `Self`
    pub fn build(
        tab_action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) -> Self {
        // Init local actions
        let action_update = SimpleAction::new(&uuid_string_random(), None);
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let control = Rc::new(Control::build(action_send.clone()));
        let form = Rc::new(Form::build(action_update.clone()));
        let title = Rc::new(Title::build(title));

        // Init main widget
        let g_box = Box::builder()
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .margin_top(MARGIN)
            .spacing(SPACING)
            .orientation(Orientation::Vertical)
            .build();

        g_box.append(&title.label);
        g_box.append(&form.text_view);
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
                tab_action.load.activate(
                    Some(&format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(&form.text(), None, false),
                    )),
                    true,
                );
            }
        });

        // Return activated struct
        Self { g_box }
    }
}
