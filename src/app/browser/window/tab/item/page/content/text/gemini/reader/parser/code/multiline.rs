use gtk::glib::GString;

pub struct Multiline {
    pub alt: Option<GString>,
    pub buffer: Vec<GString>,
    pub completed: bool,
}

impl Multiline {
    // Search in line for tag open,
    // return Self constructed on success or None
    pub fn begin_from(line: &str) -> Option<Self> {
        if line.starts_with("```") {
            let alt = line.trim_start_matches("```");

            return Some(Self {
                alt: match alt.trim().is_empty() {
                    true => None,
                    false => Some(GString::from(alt)),
                },
                buffer: Vec::new(),
                completed: false,
            });
        }

        None
    }

    // Continue preformatted buffer from line,
    // set `completed` as True on close tag found
    pub fn continue_from(&mut self, line: &str) {
        // Make sure buffer not completed yet
        if self.completed {
            panic!("Could not continue as completed") // @TODO handle
        }

        // Line contain close tag
        if line.ends_with("```") {
            self.completed = true;
        }

        // Append data to the buffer, trim close tag on exists
        self.buffer
            .push(GString::from(line.trim_end_matches("```")));
    }
}
