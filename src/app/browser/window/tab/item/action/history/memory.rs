mod cursor;

use cursor::Cursor;
use gtk::glib::GString;
use std::cell::RefCell;

pub struct Memory {
    cursor: RefCell<Cursor>,
    index: RefCell<Vec<GString>>,
}

impl Memory {
    // Constructors

    pub fn new() -> Self {
        Self {
            cursor: RefCell::new(Cursor::new()),
            index: RefCell::new(Vec::new()),
        }
    }

    // Actions

    /// Create new record in the navigation memory
    pub fn add(&self, value: GString, follow_to_index: bool) {
        // borrow subject in r/w mode
        let mut index = self.index.borrow_mut();

        if follow_to_index {
            // drop forward history if the user continue navigation
            // from the previous history position
            if let Some(next) = self.cursor.borrow_mut().next(index.len()) {
                index.truncate(next);
            }
        }

        // prevent duplicates at the last history position
        // e.g. on page reload with `follow_to_index` enabled
        match index.last() {
            Some(last) => {
                if *last != value {
                    index.push(value);
                }
            }
            None => index.push(value),
        }

        if follow_to_index {
            // set cursor on to the last record
            self.cursor.borrow_mut().go_last(index.len());
        }
    }

    pub fn back(&self, follow_to_index: bool) -> Option<GString> {
        let index = self.index.borrow();
        let len = index.len();

        match if follow_to_index {
            self.cursor.borrow_mut().go_back(len)
        } else {
            self.cursor.borrow().back(len)
        } {
            Some(i) => index.get(i).cloned(),
            None => None,
        }
    }

    pub fn next(&self, follow_to_index: bool) -> Option<GString> {
        let index = self.index.borrow();
        let len = index.len();

        match if follow_to_index {
            self.cursor.borrow_mut().go_next(len)
        } else {
            self.cursor.borrow().next(len)
        } {
            Some(i) => index.get(i).cloned(),
            None => None,
        }
    }
}
