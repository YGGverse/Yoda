mod gemini;

use gemini::Gemini;

use gtk::ScrolledWindow;

pub struct Text {
    widget: ScrolledWindow,
}

impl Text {
    // Construct
    pub fn gemini(gemtext: &str) -> Self {
        // Init components
        let gemini = Gemini::new(gemtext);

        // Init widget
        let widget = ScrolledWindow::builder().build();

        widget.set_child(Some(gemini.widget()));

        // Result
        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &ScrolledWindow {
        &self.widget
    }
}
