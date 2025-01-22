mod response;
mod sensitive;
mod titan;

use super::TabAction;
use adw::Clamp;
use gtk::{glib::Uri, prelude::WidgetExt, Box, Label};
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
        self.update(None);
    }

    pub fn update(&self, child: Option<&Box>) {
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
        action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) {
        self.update(Some(
            &Response::build(action, base, title, size_limit).g_box,
        ));
    }

    pub fn set_new_sensitive(
        &self,
        action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) {
        self.update(Some(
            &Sensitive::build(action, base, title, max_length).g_box,
        ));
    }

    pub fn set_new_titan(&self, on_send: impl Fn(&[u8], &Label) + 'static) {
        self.update(Some(&Titan::build(on_send).g_box));
    }
}
