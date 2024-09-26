mod reader;

use reader::Reader;

use gtk::Viewport;

pub struct Gemini {
    widget: Viewport,
}

impl Gemini {
    // Construct
    pub fn new(gemtext: &str) -> Self {
        // Init components
        let reader = Reader::new(gemtext);

        // Init widget
        let widget = Viewport::builder().scroll_to_focus(false).build();

        widget.set_child(Some(reader.widget()));

        // Result
        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Viewport {
        &self.widget
    }
}
