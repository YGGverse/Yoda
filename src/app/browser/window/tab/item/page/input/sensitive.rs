mod form;
mod widget;

use form::Form;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri, UriHideFlags},
    prelude::{ActionExt, ToVariant, WidgetExt},
    Box,
};
use std::sync::Arc;

pub struct Sensitive {
    // Components
    widget: Arc<Widget>,
}

impl Sensitive {
    // Construct
    pub fn new_arc(
        action_page_open: Arc<SimpleAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) -> Arc<Self> {
        // Init local actions
        let action_send = Arc::new(SimpleAction::new(&uuid_string_random(), None));

        // Init components
        let form = Form::new_arc(
            action_send.clone(),
            title,
            match max_length {
                Some(value) => {
                    Some(value - base.to_string_partial(UriHideFlags::QUERY).len() as i32)
                }
                None => None,
            },
        );

        // Init widget
        let widget = Widget::new_arc(form.gobject());

        // Init events
        action_send.connect_activate({
            let form = form.clone();
            move |_, _| {
                action_page_open.activate(Some(
                    &format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(form.text().as_str(), None, false),
                    )
                    .to_variant(),
                ));
            }
        });

        widget.gobject().connect_realize(move |_| form.focus());

        // Return activated struct
        Arc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
