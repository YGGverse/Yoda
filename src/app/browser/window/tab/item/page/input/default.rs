mod control;
mod response;
mod title;
mod widget;

use control::Control;
use response::Response;
use title::Title;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri, UriHideFlags},
    prelude::{ActionExt, ToVariant, WidgetExt},
    Box,
};
use std::sync::Arc;

pub struct Default {
    // Components
    widget: Arc<Widget>,
}

impl Default {
    // Construct
    pub fn new_arc(
        action_page_open: Arc<SimpleAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) -> Arc<Self> {
        // Init local actions
        let action_update = Arc::new(SimpleAction::new(&uuid_string_random(), None));
        let action_send = Arc::new(SimpleAction::new(&uuid_string_random(), None));

        // Init components
        let control = Control::new_arc(action_send.clone());
        let response = Response::new_arc(action_update.clone());
        let title = Title::new_arc(title);

        // Init widget
        let widget = Widget::new_arc(title.gobject(), response.gobject(), control.gobject());

        // Init events
        action_update.connect_activate({
            let base = base.clone();
            let control = control.clone();
            let response = response.clone();
            move |_, _| {
                control.update(match size_limit {
                    Some(limit) => Some(
                        limit as i32
                            - (base.to_string_partial(UriHideFlags::QUERY).len() as i32
                                + Uri::escape_string(response.text().as_str(), None, false).len()
                                    as i32),
                    ),
                    None => None,
                });
            }
        });

        action_send.connect_activate({
            let response = response.clone();
            move |_, _| {
                action_page_open.activate(Some(
                    &format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(response.text().as_str(), None, false),
                    )
                    .to_variant(),
                ));
            }
        });

        widget.gobject().connect_realize(move |_| response.focus());

        // Return activated struct
        Arc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
