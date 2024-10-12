mod back;
mod forward;
mod widget;

use back::Back;
use forward::Forward;
use widget::Widget;

use gtk::{gio::SimpleAction, glib::GString, Box};
use std::{cell::RefCell, sync::Arc};

struct Memory {
    request: GString,
    // time: SystemTime,
}

pub struct History {
    // Components
    back: Arc<Back>,
    forward: Arc<Forward>,
    // Extras
    memory: RefCell<Vec<Memory>>,
    index: RefCell<Option<usize>>,
    // GTK
    widget: Arc<Widget>,
}

impl History {
    // Construct
    pub fn new_arc(
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
    ) -> Arc<Self> {
        // init components
        let back = Back::new_arc(action_tab_page_navigation_history_back);
        let forward = Forward::new_arc(action_tab_page_navigation_history_forward);

        // Init widget
        let widget = Widget::new_arc(back.gobject(), forward.gobject());

        // Init memory
        let memory = RefCell::new(Vec::new());

        // Init index
        let index = RefCell::new(None);

        Arc::new(Self {
            back,
            forward,
            memory,
            index,
            widget,
        })
    }

    // Actions
    pub fn add(&self, request: GString, follow_to_index: bool) {
        // Append new Memory record
        self.memory.borrow_mut().push(Memory {
            request: request.clone(),
            //time: SystemTime::now(),
        });

        if follow_to_index {
            // Even push action make positive len value, make sure twice
            if !self.memory.borrow().is_empty() {
                // Navigate to the last record appended
                self.index.replace(Some(self.memory.borrow().len() - 1));
            } else {
                self.index.replace(None);
            }
        }
    }

    pub fn back(&self, follow_to_index: bool) -> Option<GString> {
        let index = self.index.borrow().clone(); // keep outside as borrow
        if let Some(usize) = index {
            // Make sure value positive to prevent panic
            if usize > 0 {
                if let Some(memory) = self.memory.borrow().get(usize - 1) {
                    if follow_to_index {
                        self.index.replace(Some(usize - 1));
                    }
                    return Some(memory.request.clone());
                }
            }
        }
        None
    }

    pub fn current(&self) -> Option<GString> {
        let index = self.index.borrow().clone(); // keep outside as borrow
        if let Some(usize) = index {
            if let Some(memory) = self.memory.borrow().get(usize) {
                return Some(memory.request.clone());
            }
        }
        None
    }

    pub fn forward(&self, follow_to_index: bool) -> Option<GString> {
        let index = self.index.borrow().clone(); // keep outside as borrow
        if let Some(usize) = index {
            if let Some(memory) = self.memory.borrow().get(usize + 1) {
                if follow_to_index {
                    self.index.replace(Some(usize + 1));
                }
                return Some(memory.request.clone());
            }
        }
        None
    }

    pub fn update(&self) {
        match self.back(false) {
            Some(_) => self.back.update(true),
            None => self.back.update(false),
        };

        match self.forward(false) {
            Some(_) => self.forward.update(true),
            None => self.forward.update(false),
        };
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
