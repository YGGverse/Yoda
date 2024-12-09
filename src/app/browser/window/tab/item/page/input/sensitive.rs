mod form;
mod widget;

use form::Form;
use widget::Widget;

use crate::app::browser::window::tab::item::action::Action as TabAction;
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri, UriHideFlags},
    prelude::{EditableExt, WidgetExt},
    Box,
};
use std::rc::Rc;

pub struct Sensitive {
    // Components
    widget: Rc<Widget>,
}

impl Sensitive {
    // Construct
    pub fn new(
        tab_action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) -> Self {
        // Init local actions
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let form = Rc::new(Form::new(
            action_send.clone(),
            title,
            max_length
                .map(|value| value - base.to_string_partial(UriHideFlags::QUERY).len() as i32),
        ));

        // Init widget
        let widget = Rc::new(Widget::new(&form.widget.password_entry_row));

        // Init events
        action_send.connect_activate({
            let form = form.clone();
            move |_, _| {
                tab_action.load.activate(
                    Some(&format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(&form.widget.password_entry_row.text(), None, false),
                    )),
                    true,
                );
            }
        });

        widget.gobject().connect_realize(move |_| {
            form.widget.password_entry_row.grab_focus();
        });

        // Return activated struct
        Self { widget }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        self.widget.gobject()
    }
}
