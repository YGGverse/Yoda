mod control;
mod form;
mod title;

use control::Control;
use form::Form;
use title::Title;

use gtk::{gio::SimpleAction, glib::uuid_string_random, prelude::BoxExt, Box, Label, Orientation};
use std::rc::Rc;

const MARGIN: i32 = 6;
const SPACING: i32 = 8;

pub struct Titan {
    // Components
    pub g_box: Box,
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
            let control = control.clone();
            let form = form.clone();
            move |_, _| control.update(Some(form.text().len()))
        });

        action_send
            .connect_activate(move |_, _| on_send(form.text().as_bytes(), &control.counter.label));

        // Return activated struct
        Self { g_box }
    }
}
