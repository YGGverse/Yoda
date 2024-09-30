mod back;
mod forward;

use back::Back;
use forward::Forward;

use gtk::{gio::SimpleAction, glib::GString, prelude::BoxExt, Box, Orientation};
use std::{cell::RefCell, sync::Arc};

struct Memory {
    request: GString,
    time: i32, // @TODO
}

pub struct History {
    // Components
    back: Back,
    forward: Forward,
    // Extras
    memory: Vec<Memory>,
    index: RefCell<i32>,
    // GTK
    widget: Box,
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

        // Init memory
        let memory = Vec::new();

        // Init index
        let index = RefCell::new(-1);

        Self {
            // Actions
            back,
            forward,
            // Extras
            memory,
            index,
            // GTK
            widget,
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
