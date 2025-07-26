use std::rc::Rc;

use super::Profile;
use adw::{PreferencesGroup, SwitchRow, prelude::PreferencesGroupExt};

pub struct Misc {
    highlight_request_entry: SwitchRow,
    pub widget: PreferencesGroup,
}

impl Misc {
    // Constructors

    pub fn build(profile: &Rc<Profile>) -> Self {
        // Init components

        let highlight_request_entry = SwitchRow::builder()
            .active(profile.proxy.misc.is_highlight_request_entry())
            .hexpand(true)
            .subtitle_selectable(true)
            .subtitle("Indicate proxy connections with accent colors")
            .title_selectable(true)
            .title("Highlight the Request entry")
            .build();

        // Init widget

        let widget = PreferencesGroup::new();

        widget.add(&highlight_request_entry);

        Self {
            highlight_request_entry,
            widget,
        }
    }

    pub fn is_highlight_request_entry(&self) -> bool {
        self.highlight_request_entry.is_active()
    }
}
