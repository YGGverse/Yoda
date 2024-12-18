pub struct Cursor {
    current: usize,
    last: usize,
}

impl Cursor {
    pub fn new(len: usize) -> Self {
        Self {
            current: 0,
            last: len,
        }
    }

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

    pub fn as_position(&self) -> usize {
        self.current
    }
}
