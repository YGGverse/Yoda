mod control;
mod form;
mod title;
mod widget;

use control::Control;
use form::Form;
use title::Title;
use widget::Widget;

use crate::app::browser::window::tab::action::Action as TabAction;
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri, UriHideFlags},
    prelude::WidgetExt,
    Box,
};
use std::rc::Rc;

pub struct Response {
    // Components
    widget: Rc<Widget>,
}

impl Response {
    // Construct
    pub fn new_rc(
        tab_action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) -> Rc<Self> {
        // Init local actions
        let action_update = SimpleAction::new(&uuid_string_random(), None);
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let control = Control::new_rc(action_send.clone());
        let form = Form::new_rc(action_update.clone());
        let title = Title::new_rc(title);

        // Init widget
        let widget = Widget::new_rc(title.gobject(), form.gobject(), control.gobject());

        // Init events
        action_update.connect_activate({
            let base = base.clone();
            let control = control.clone();
            let form = form.clone();
            move |_, _| {
                control.update(size_limit.map(|limit| {
                    limit as i32
                        - (base.to_string_partial(UriHideFlags::QUERY).len() as i32
                            + Uri::escape_string(form.text().as_str(), None, false).len() as i32)
                }));
            }
        });

        action_send.connect_activate({
            let form = form.clone();
            move |_, _| {
                tab_action.open().activate(Some(&format!(
                    "{}?{}",
                    base.to_string_partial(UriHideFlags::QUERY),
                    Uri::escape_string(form.text().as_str(), None, false),
                )));
            }
        });

        widget.gobject().connect_realize(move |_| form.focus());

        // Return activated struct
        Rc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        self.widget.gobject()
    }
}
