// @TODO mod image;
mod text;

use text::Text;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};

use std::sync::Arc;

pub enum Mime {
    TextGemini,
    // TextPlain,
}

pub struct ResetResult {
    pub title: Option<GString>,
}

pub struct Content {
    // GTK
    widget: Box,
    // Actions
    action_tab_open: Arc<SimpleAction>,
    action_page_open: Arc<SimpleAction>,
}

impl Content {
    // Construct
    pub fn new(action_tab_open: Arc<SimpleAction>, action_page_open: Arc<SimpleAction>) -> Self {
        Self {
            widget: Box::builder().orientation(Orientation::Vertical).build(),
            action_tab_open,
            action_page_open,
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
                let child = Text::gemini(
                    data,
                    base,
                    self.action_tab_open.clone(),
                    self.action_page_open.clone(),
                );

                self.widget.append(child.widget());

                ResetResult {
                    title: child.meta_title().clone(),
                }
            } /* @TODO
              Mime::TextPlain => {
                  todo!()
              } */
        }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget
    }
}
