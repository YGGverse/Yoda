struct Cursor {
    current: usize,
    last: usize,
}

impl Cursor {
    pub fn back(&mut self) {
        self.current = if self.current > 0 {
            self.current - 1
        } else {
            self.last
        }
    }

    pub fn next(&mut self) {
        self.current = if self.current < self.last {
            self.current + 1
        } else {
            0
        }
    }

    pub fn as_index(&self) -> usize {
        if self.current > 0 {
            self.current - 1
        } else {
            0
        }
    }
}

pub struct Model<T> {
    cursor: Cursor,
    vector: Vec<T>,
}

impl<T> Model<T> {
    pub fn new(vector: Vec<T>) -> Self {
        Self {
            cursor: Cursor {
                current: 0,
                last: vector.len(),
            },
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
