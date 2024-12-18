mod cursor;
use cursor::Cursor;

pub struct Model<T> {
    cursor: Cursor,
    vector: Vec<T>,
}

impl<T> Model<T> {
    // Constructors
    pub fn new(vector: Vec<T>) -> Self {
        Self {
            cursor: Cursor::new(vector.len()),
            vector,
        }
    }

    // Actions

    pub fn back(&mut self) -> Option<&T> {
        self.cursor.back();
        self.vector.get(self.cursor.as_index())
    }

    pub fn next(&mut self) -> Option<&T> {
        self.cursor.next();
        self.vector.get(self.cursor.as_index())
    }

    // Getters

    pub fn position(&self) -> Option<usize> {
        self.cursor.as_position()
    }

    pub fn total(&self) -> usize {
        self.vector.len()
    }

    pub fn is_back(&self) -> bool {
        self.cursor.is_back()
    }

    pub fn is_next(&self) -> bool {
        self.cursor.is_next()
    }
}
