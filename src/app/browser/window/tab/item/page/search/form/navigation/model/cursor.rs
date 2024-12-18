pub struct Cursor {
    current: usize,
    last: usize,
}

impl Cursor {
    // Constructors

    pub fn new(len: usize) -> Self {
        Self {
            current: 0,
            last: len,
        }
    }

    // Actions

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

    // Getters

    pub fn as_index(&self) -> usize {
        if self.current > 0 {
            self.current - 1
        } else {
            0
        }
    }

    pub fn as_position(&self) -> Option<usize> {
        if self.current > 0 {
            Some(self.current)
        } else {
            None
        }
    }

    pub fn is_back(&self) -> bool {
        self.current > 0
    }

    pub fn is_next(&self) -> bool {
        self.current < self.last
    }
}
