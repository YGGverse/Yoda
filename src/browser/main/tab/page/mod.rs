mod content;
mod navigation;

use content::Content;
use navigation::Navigation;

use gtk::{glib::GString, prelude::BoxExt, Box, Orientation};

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
        self.content.reload(self.navigation.request_text());
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
