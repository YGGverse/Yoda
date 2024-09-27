// @TODO mod image;
mod text;

use text::Text;

use gtk::{
    glib::{GString, Uri},
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};

pub enum Mime {
    TextGemini,
    TextPlain,
}

pub struct ResetResult {
    pub title: Option<GString>,
}

pub struct Content {
    widget: Box,
}

impl Content {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Box::builder().orientation(Orientation::Vertical).build(),
        }
    }

    // Actions
    pub fn reset(&self, mime: Mime, base: &Uri, data: &str) -> ResetResult {
        // Cleanup
        while let Some(child) = self.widget.last_child() {
            self.widget.remove(&child)
        }

        // Re-compose
        match mime {
            Mime::TextGemini => {
                let child = Text::gemini(data, base);

                self.widget.append(child.widget());

                ResetResult {
                    title: child.meta_title().clone(),
                }
            }
            Mime::TextPlain => {
                todo!()
            }
        }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
