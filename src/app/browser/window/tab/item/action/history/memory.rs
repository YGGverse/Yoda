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

    pub fn add(&self, value: GString, follow_to_index: bool) {
        let mut index = self.index.borrow_mut();

        match index.last() {
            Some(last) => {
                if *last != value {
                    index.push(value);
                }
            }
            None => index.push(value),
        }

        if follow_to_index {
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
