// mod database;
mod memory;

use memory::Memory;

use sqlite::Connection;
use std::{rc::Rc, sync::RwLock};

pub struct History {
    pub memory: Rc<Memory>, // fast search index
}

impl History {
    // Constructors

    /// Create new `Self`
    pub fn build(_connection: &Rc<RwLock<Connection>>, _profile_id: &Rc<i64>) -> Self {
        // Init children components
        let memory = Rc::new(Memory::new());

        // Return new `Self`
        Self { memory }
    }
}
