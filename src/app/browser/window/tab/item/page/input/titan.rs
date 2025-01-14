mod control;
mod form;
mod title;
mod widget;

use control::Control;
use form::Form;
use title::Title;
use widget::Widget;

use super::TabAction;
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Uri},
};
use std::rc::Rc;

pub struct Titan {
    // Components
    pub widget: Rc<Widget>,
}

impl Titan {
    // Constructors

    /// Build new `Self`
    pub fn build(_tab_action: Rc<TabAction>, _base: Uri, title: Option<&str>) -> Self {
        // Init local actions
        let action_update = SimpleAction::new(&uuid_string_random(), None);
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let control = Rc::new(Control::build(action_send.clone()));
        let form = Rc::new(Form::build(action_update.clone()));
        let title = Rc::new(Title::build(title));

        // Init widget
        let widget = Rc::new(Widget::build(
            &title.widget.label,
            &form.widget.text_view,
            &control.widget.g_box,
        ));

        // Init events
        action_update.connect_activate({
            let control = control.clone();
            let form = form.clone();
            move |_, _| control.update(Some(form.widget.size()))
        });

        action_send.connect_activate({
            // @TODO let form = form.clone();
            move |_, _| {
                todo!()
                /* tab_action.load.activate(
                    Some(&),
                    true,
                );*/
            }
        });

        // Return activated struct
        Self { widget }
    }
}
