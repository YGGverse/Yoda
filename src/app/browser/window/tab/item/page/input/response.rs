mod control;
mod form;
mod title;
mod widget;

use control::Control;
use form::Form;
use title::Title;
use widget::Widget;

use crate::app::browser::window::tab::item::action::Action as TabAction;
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri, UriHideFlags},
    prelude::WidgetExt,
};
use std::rc::Rc;

pub struct Response {
    // Components
    pub widget: Rc<Widget>,
}

impl Response {
    // Construct
    pub fn new(
        tab_action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) -> Self {
        // Init local actions
        let action_update = SimpleAction::new(&uuid_string_random(), None);
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let control = Rc::new(Control::new(action_send.clone()));
        let form = Rc::new(Form::new(action_update.clone()));
        let title = Rc::new(Title::new(title));

        // Init widget
        let widget = Rc::new(Widget::new(
            &title.widget.label,
            &form.widget.text_view,
            &control.widget.g_box,
        ));

        // Init events
        action_update.connect_activate({
            let base = base.clone();
            let control = control.clone();
            let form = form.clone();
            move |_, _| {
                control.update(size_limit.map(|limit| {
                    limit as i32
                        - (base.to_string_partial(UriHideFlags::QUERY).len() as i32
                            + Uri::escape_string(&form.widget.text(), None, false).len() as i32)
                }));
            }
        });

        action_send.connect_activate({
            let form = form.clone();
            move |_, _| {
                tab_action.load().activate(
                    Some(&format!(
                        "{}?{}",
                        base.to_string_partial(UriHideFlags::QUERY),
                        Uri::escape_string(&form.widget.text(), None, false),
                    )),
                    true,
                );
            }
        });

        widget.g_box.connect_realize(move |_| {
            form.widget.text_view.grab_focus();
        });

        // Return activated struct
        Self { widget }
    }
}
