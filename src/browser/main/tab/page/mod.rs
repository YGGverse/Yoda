mod content;
mod navigation;

use content::Content;
use navigation::Navigation;

use gtk::{
    glib::{GString, Regex, RegexCompileFlags, RegexMatchFlags, Uri, UriFlags},
    prelude::BoxExt,
    Box, Orientation,
};

pub struct Page {
    widget: Box,
    navigation: Navigation,
    content: Content,
}

impl Page {
    // Construct
    pub fn new(name: GString) -> Page {
        // Init components
        let content = Content::new();
        let navigation = Navigation::new();

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        widget.append(navigation.widget());
        widget.append(content.widget());

        // Result
        Self {
            widget,
            content,
            navigation,
        }
    }

    // Actions
    pub fn reload(&self) {
        let request_text = self.navigation.request_text();
        /*let _uri = */
        match Uri::parse(&request_text, UriFlags::NONE) {
            Ok(uri) => {
                println!("Parsed URI: {}", uri); // @TODO
            }
            Err(_) => {
                // Try interpret host manually
                if Regex::match_simple(
                    r"regex(^[^\/\s]+\.[\w]{2,})regex",
                    request_text.clone(),
                    RegexCompileFlags::DEFAULT,
                    RegexMatchFlags::DEFAULT,
                ) {
                    // Seems request contain some host substring
                    self.navigation.set_request_text(
                        &GString::from(format!("gemini://{request_text}")),
                        true, // activate (page reload) @TODO validate new uri instead?
                    );
                } else {
                    // Plain text given, make search request to default provider
                    Uri::escape_string(&request_text, None, false);
                    self.navigation.set_request_text(
                        &GString::from(format!("gemini://tlgs.one/search?{request_text}")),
                        true, // activate (page reload)
                    );
                }
            }
        };
    }

    pub fn update(&self) {
        self.navigation.update();
        // @TODO self.content.update();
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
