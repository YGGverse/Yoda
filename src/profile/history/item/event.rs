use gtk::glib::DateTime;

#[derive(Clone)]
pub struct Event {
    pub time: DateTime,
    pub count: usize,
}

impl Event {
    // Constructors

    pub fn new() -> Self {
        Self {
            time: now(),
            count: 1,
        }
    }

    // Actions

    pub fn pulse(&mut self) {
        self.time = now();
        self.count += 1;
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}

// Tools

fn now() -> DateTime {
    DateTime::now_local().unwrap()
}
