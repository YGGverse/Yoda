mod control;
mod form;
mod title;
mod widget;

use control::Control;
use form::Form;
use title::Title;
use widget::Widget;

use gtk::{gio::SimpleAction, glib::uuid_string_random, Label};
use std::rc::Rc;

pub struct Titan {
    // Components
    pub widget: Rc<Widget>,
}

impl Titan {
    // Constructors

    /// Build new `Self`
    pub fn build(on_send: impl Fn(&[u8], &Label) + 'static) -> Self {
        // Init local actions
        let action_update = SimpleAction::new(&uuid_string_random(), None);
        let action_send = SimpleAction::new(&uuid_string_random(), None);

        // Init components
        let control = Rc::new(Control::build(action_send.clone()));
        let form = Rc::new(Form::build(action_update.clone()));
        let title = Title::build(None);

        // Init widget
        let widget = Rc::new(Widget::build(
            &title.label,
            &form.widget.text_view,
            &control.widget.g_box,
        ));

        // Init events
        action_update.connect_activate({
            let control = control.clone();
            let form = form.clone();
            move |_, _| control.update(Some(form.widget.text().as_bytes().len()))
        });

        action_send.connect_activate(move |_, _| {
            on_send(form.widget.text().as_bytes(), &control.counter.label)
        });

        // Return activated struct
        Self { widget }
    }
}
