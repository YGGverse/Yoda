mod back;
mod forward;

use back::Back;
use forward::Forward;

use gtk::{gio::SimpleAction, prelude::BoxExt, Box, Orientation};
use std::sync::Arc;

pub struct History {
    widget: Box,
    back: Back,
    forward: Forward,
}

impl History {
    // Construct
    pub fn new(
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
    ) -> Self {
        // init components
        let back = Back::new(action_tab_page_navigation_history_back);
        let forward = Forward::new(action_tab_page_navigation_history_forward);

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        widget.append(back.widget());
        widget.append(forward.widget());

        Self {
            widget,
            back,
            forward,
        }
    }

    // Actions
    pub fn update(&self) {
        self.back.update();
        self.forward.update();
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
