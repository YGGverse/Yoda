mod back;
mod forward;

use back::Back;
use forward::Forward;

use gtk::{gio::SimpleAction, glib::GString, prelude::BoxExt, Box, Orientation};
use std::{cell::RefCell, sync::Arc};

struct Memory {
    request: GString,
    // time: SystemTime,
}

pub struct History {
    // Components
    back: Back,
    forward: Forward,
    // Extras
    memory: RefCell<Vec<Memory>>,
    index: RefCell<Option<usize>>,
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
        let memory = RefCell::new(Vec::new());

        // Init index
        let index = RefCell::new(None);

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
    pub fn add(&self, request: GString, follow_to_index: bool) {
        // Append new Memory record
        self.memory.borrow_mut().push(Memory {
            request,
            //time: SystemTime::now(),
        });

        if follow_to_index {
            // Navigate to the last record appended
            self.index.replace(Some(self.memory.borrow().len()));
        }
    }

    pub fn try_back(&self, follow_to_index: bool) -> Option<GString> {
        if let Some(index) = self.index.borrow().as_ref() {
            if let Some(memory) = self.memory.borrow().get(index - 1) {
                if follow_to_index {
                    self.index.replace(Some(index - 1));
                }
                return Some(memory.request.clone());
            }
        }
        None
    }

    pub fn try_forward(&self, follow_to_index: bool) -> Option<GString> {
        if let Some(index) = self.index.borrow().as_ref() {
            if let Some(memory) = self.memory.borrow().get(index + 1) {
                if follow_to_index {
                    self.index.replace(Some(index + 1));
                }
                return Some(memory.request.clone());
            }
        }
        None
    }

    pub fn update(&self) {
        match self.try_back(false) {
            Some(_) => self.back.update(true),
            None => self.back.update(false),
        };

        match self.try_forward(false) {
            Some(_) => self.forward.update(true),
            None => self.forward.update(false),
        };
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
