mod back;
mod forward;
mod memory;

use back::Back;
use forward::Forward;
use gtk::{gio::SimpleAction, glib::GString};
use memory::Memory;
use std::rc::Rc;

pub struct History {
    memory: Rc<Memory>,
    pub back: SimpleAction,
    pub forward: SimpleAction,
}

impl History {
    // Constructors

    /// Build new activated `Self`
    pub fn build(callback: impl Fn(GString) + 'static) -> Self {
        // Init childs
        let memory = Rc::new(Memory::new());
        let back = SimpleAction::back();
        let forward = SimpleAction::forward();

        // Init events
        let callback = Rc::new(callback);

        back.connect_activate({
            let callback = callback.clone();
            let forward = forward.clone();
            let memory = memory.clone();
            move |this, _| {
                if let Some(request) = memory.back(true) {
                    callback(request)
                }
                forward.set_enabled(memory.next(false).is_some());
                this.set_enabled(memory.back(false).is_some());
            }
        });

        forward.connect_activate({
            let back = back.clone();
            let callback = callback.clone();
            let memory = memory.clone();
            move |this, _| {
                if let Some(request) = memory.next(true) {
                    callback(request)
                }
                back.set_enabled(memory.back(false).is_some());
                this.set_enabled(memory.next(false).is_some());
            }
        });

        // Done
        Self {
            memory,
            back,
            forward,
        }
    }

    // Actions

    pub fn add(&self, request: GString, follow_to_index: bool) {
        self.memory.add(request, follow_to_index);
        self.back.set_enabled(self.back(false).is_some());
        self.forward.set_enabled(self.forward(false).is_some());
    }

    pub fn back(&self, follow_to_index: bool) -> Option<GString> {
        self.memory.back(follow_to_index)
    }

    pub fn forward(&self, follow_to_index: bool) -> Option<GString> {
        self.memory.next(follow_to_index)
    }
}
