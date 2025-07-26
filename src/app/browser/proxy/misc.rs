use std::rc::Rc;

use super::Profile;
use adw::SwitchRow;
use gtk::{Box, prelude::BoxExt};

pub struct Misc {
    highlight_request_entry: SwitchRow,
    pub widget: Box,
}

impl Misc {
    // Constructors

    pub fn build(profile: &Rc<Profile>) -> Self {
        // Init components

        let highlight_request_entry = SwitchRow::builder()
            .active(profile.proxy.misc.is_highlight_request_entry())
            .hexpand(true)
            .subtitle("Use accent color for proxy connections")
            .title("Highlight the Request entry")
            //.subtitle_selectable(true)
            //.title_selectable(true)
            .build();

        // Init widget

        let widget = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();

        widget.append(&highlight_request_entry);

        Self {
            highlight_request_entry,
            widget,
        }
    }

    pub fn is_highlight_request_entry(&self) -> bool {
        self.highlight_request_entry.is_active()
    }
}
