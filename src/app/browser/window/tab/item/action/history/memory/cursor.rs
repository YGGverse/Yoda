pub struct Cursor(Option<usize>);

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    // Constructors

    pub fn new() -> Self {
        Self(None)
    }

    // Actions

    pub fn go_last(&mut self, len: usize) -> Option<usize> {
        self.0 = len2i(len);
        self.0
    }

    pub fn go_next(&mut self, len: usize) -> Option<usize> {
        self.0 = self.next(len);
        self.0
    }

    pub fn go_back(&mut self, len: usize) -> Option<usize> {
        self.0 = self.back(len);
        self.0
    }

    // Getters

    pub fn next(&self, len: usize) -> Option<usize> {
        let i = len2i(len)?;
        let n = self.0.unwrap_or_default();

        if n < i { Some(n + 1) } else { None }
    }

    pub fn back(&self, len: usize) -> Option<usize> {
        len2i(len)?;
        let n = self.0.unwrap_or_default();

        if n > 0 { Some(n - 1) } else { None }
    }
}

// Tools

fn len2i(len: usize) -> Option<usize> {
    if len > 0 { Some(len - 1) } else { None }
}
