mod back;
mod forward;
mod widget;

use back::Back;
use forward::Forward;
use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::glib::GString;
use std::{cell::RefCell, rc::Rc};

struct Memory {
    request: GString,
    // time: SystemTime,
}

pub struct History {
    // Components
    back: Rc<Back>,
    forward: Rc<Forward>,
    // Extras
    memory: RefCell<Vec<Memory>>,
    index: RefCell<Option<usize>>,
    // GTK
    widget: Rc<Widget>,
}

impl History {
    // Construct
    pub fn new(window_action: Rc<WindowAction>) -> Self {
        // init components
        let back = Rc::new(Back::new(window_action.clone()));
        let forward = Rc::new(Forward::new(window_action));

        // Init widget
        let widget = Rc::new(Widget::new(
            back.widget().gobject(),
            forward.widget().gobject(),
        ));

        // Init memory
        let memory = RefCell::new(Vec::new());

        // Init index
        let index = RefCell::new(None);

        Self {
            back,
            forward,
            memory,
            index,
            widget,
        }
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
        let index = *self.index.borrow();
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
        let index = *self.index.borrow();
        if let Some(usize) = index {
            if let Some(memory) = self.memory.borrow().get(usize) {
                return Some(memory.request.clone());
            }
        }
        None
    }

    pub fn forward(&self, follow_to_index: bool) -> Option<GString> {
        let index = *self.index.borrow();
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

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
    }
}
