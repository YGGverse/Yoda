use gtk::TextIter;

pub struct Iter {
    value: Vec<(TextIter, TextIter)>,
    index: Option<usize>,
}

impl Iter {
    pub fn new(value: Vec<(TextIter, TextIter)>) -> Self {
        Self { index: None, value }
    }

    pub fn back(&mut self) -> Option<(TextIter, TextIter)> {
        self.index = match self.index {
            Some(index) => {
                if index > 0 {
                    Some(index - 1)
                } else {
                    Some(self.value.len())
                }
            }
            None => Some(self.value.len()), // init
        };
        self.value.get(self.index.unwrap_or_default()).copied()
    }

    pub fn forward(&mut self) -> Option<(TextIter, TextIter)> {
        self.index = match self.index {
            Some(index) => {
                if index < self.value.len() {
                    Some(index + 1)
                } else {
                    Some(0)
                }
            }
            None => Some(0), // init
        };
        self.value.get(self.index.unwrap_or_default()).copied()
    }

    pub fn reset(&mut self) -> Option<(TextIter, TextIter)> {
        self.index = None;
        self.forward()
    }
}
