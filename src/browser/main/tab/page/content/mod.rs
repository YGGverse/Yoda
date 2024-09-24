use gtk::{
    glib::{GString, Regex, RegexCompileFlags, RegexMatchFlags, Uri, UriFlags},
    Box, Orientation,
};

pub struct Content {
    widget: Box,
}

impl Content {
    // Construct
    pub fn new() -> Content {
        Self {
            widget: Box::builder().orientation(Orientation::Vertical).build(),
        }
    }

    // Actions
    pub fn reload(&self, request_text: GString) {
        /*let _uri = */
        match Uri::parse(&request_text, UriFlags::NONE) {
            Ok(uri) => {
                println!("Parsed URI: {}", uri); // @TODO
            }
            Err(_) => {
                // Request contain host substring
                if Regex::match_simple(
                    r"regex(^[^\/\s]+\.[\w]{2,})regex",
                    request_text.clone(),
                    RegexCompileFlags::DEFAULT,
                    RegexMatchFlags::DEFAULT,
                ) {
                    let request_text = format!("gemini://{request_text}");
                    // @TODO reload
                } else {
                    Uri::escape_string(&request_text, None, false);
                    let request_text = format!("gemini://tlgs.one/search?{request_text}");
                    // @TODO reload
                }
            }
        };
        // @TODO
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
