use super::WidgetAction;
use gtk::{
    prelude::{EditableExt, EntryExt, WidgetExt},
    Entry,
};
use std::rc::Rc;

const MIN_LENGTH: u16 = 1;
const MAX_LENGTH: u16 = 36;

pub trait Name {
    // Constructors

    fn name(widget_action: &Rc<WidgetAction>) -> Self;

    // Actions

    fn update(&self, is_visible: bool);

    // Getters

    fn is_valid(&self) -> bool;
}

impl Name for Entry {
    // Constructors

    /// Create new `Self`
    fn name(widget_action: &Rc<WidgetAction>) -> Self {
        const PLACEHOLDER_TEXT: &str = "Identity name (required)";
        const MARGIN: i32 = 8;

        // Init main gobject
        let entry = Entry::builder()
            .margin_top(MARGIN)
            .max_length(MAX_LENGTH as i32)
            .placeholder_text(PLACEHOLDER_TEXT)
            .visible(false)
            .build();

        // Init events
        entry.connect_changed({
            let widget_action = widget_action.clone();
            move |_| widget_action.update.activate()
        });

        // Return activated `Self`
        entry
    }

    // Actions

    /// Change visibility status
    /// * grab focus on `is_visible` is `true`
    fn update(&self, is_visible: bool) {
        self.set_visible(is_visible);
        if is_visible && self.focus_child().is_none() {
            self.grab_focus();
        }
    }

    // Getters

    fn is_valid(&self) -> bool {
        self.text_length() >= MIN_LENGTH && self.text_length() <= MAX_LENGTH
    }
}
