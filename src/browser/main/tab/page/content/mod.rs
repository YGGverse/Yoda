// @TODO mod image;
mod text;

use text::Text;

use gtk::{
    glib::Uri,
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};

pub enum Mime {
    Undefined,
    TextGemini,
    TextPlain,
}

pub struct Content {
    mime: Mime,
    widget: Box,
}

impl Content {
    // Construct
    pub fn new() -> Self {
        Self {
            mime: Mime::Undefined,
            widget: Box::builder().orientation(Orientation::Vertical).build(),
        }
    }

    // Actions
    pub fn reset(&self, mime: Mime, base: &Uri, data: &str) {
        // Cleanup
        while let Some(child) = self.widget.last_child() {
            self.widget.remove(&child)
        }

        // Compose
        match mime {
            Mime::TextGemini => {
                self.widget.append(Text::gemini(data, base).widget());
            }
            Mime::TextPlain => {
                todo!()
            }
            Mime::Undefined => {
                todo!()
            }
        }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
