mod response;
mod sensitive;
mod titan;

use super::ItemAction;
use adw::Clamp;
use gtk::{
    Widget,
    glib::{Bytes, Uri},
    prelude::{IsA, WidgetExt},
};
use response::Response;
use sensitive::Sensitive;
use std::rc::Rc;
use titan::Titan;

const MARGIN: i32 = 6;

pub struct Input {
    pub clamp: Clamp,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    // Construct
    pub fn new() -> Self {
        let clamp = Clamp::builder()
            .margin_bottom(MARGIN)
            .margin_top(MARGIN)
            .maximum_size(800)
            .visible(false)
            .build();

        Self { clamp }
    }

    // Actions
    pub fn unset(&self) {
        self.update(Widget::NONE);
    }

    pub fn update(&self, child: Option<&impl IsA<Widget>>) {
        if child.is_some() {
            self.clamp.set_visible(true); // widget may be hidden, make it visible to child redraw
            self.clamp.set_child(child);
        } else {
            self.clamp.set_visible(false)
        }
    }

    // Setters
    pub fn set_new_response(
        &self,
        action: Rc<ItemAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) {
        self.update(Some(&gtk::Box::response(
            action,
            base,
            title,
            size_limit,
            MAX_CONTENT_HEIGHT,
        )));
    }

    pub fn set_new_sensitive(
        &self,
        action: Rc<ItemAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) {
        self.update(Some(&gtk::Box::sensitive(action, base, title, max_length)));
    }

    pub fn set_new_titan(&self, on_send: impl Fn(titan::Header, Bytes, Box<dyn Fn()>) + 'static) {
        self.update(Some(&gtk::Box::titan(MAX_CONTENT_HEIGHT, on_send)));
    }
}

/// @TODO optional, maybe relative to the current window height in %
const MAX_CONTENT_HEIGHT: i32 = 280;
