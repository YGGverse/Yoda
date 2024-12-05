use super::Action;
use gtk::{
    glib::GString,
    prelude::{EditableExt, EntryExt, WidgetExt},
    Entry,
};
use std::rc::Rc;

const PLACEHOLDER_TEXT: &str = "Identity name (required)";
const MARGIN: i32 = 8;
const MIN_LENGTH: u16 = 1;
const MAX_LENGTH: u16 = 36;

pub struct Name {
    pub entry: Entry,
}

impl Name {
    // Constructors

    /// Create new `Self`
    pub fn new(action: Rc<Action>) -> Self {
        // Init main gobject
        let entry = Entry::builder()
            .margin_top(MARGIN)
            .max_length(MAX_LENGTH as i32)
            .placeholder_text(PLACEHOLDER_TEXT)
            .visible(false)
            .build();

        // Init events
        entry.connect_changed(move |_| action.update.activate());

        // Return activated `Self`
        Self { entry }
    }

    // Actions

    /// Change visibility status
    /// * grab focus on `is_visible`
    pub fn update(&self, is_visible: bool) {
        self.entry.set_visible(is_visible);
        if is_visible {
            self.entry.grab_focus();
        }
    }

    // Getters

    pub fn is_valid(&self) -> bool {
        self.entry.text_length() >= MIN_LENGTH && self.entry.text_length() <= MAX_LENGTH
    }

    pub fn value(&self) -> Option<GString> {
        let text = self.entry.text();
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}
