mod cursor;
use cursor::Cursor;

pub struct Model<T> {
    cursor: Cursor,
    vector: Vec<T>,
}

impl<T> Model<T> {
    pub fn new(vector: Vec<T>) -> Self {
        Self {
            cursor: Cursor::new(vector.len()),
            vector,
        }
    }

    pub fn back(&mut self) -> Option<&T> {
        self.cursor.back();
        self.vector.get(self.cursor.as_index())
    }

    pub fn next(&mut self) -> Option<&T> {
        self.cursor.next();
        self.vector.get(self.cursor.as_index())
    }
}
