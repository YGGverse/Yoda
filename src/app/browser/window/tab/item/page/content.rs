// @TODO mod image;
mod status;
mod text;

use status::Status;
use text::Text;

use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};

use std::sync::Arc;

pub struct Content {
    // GTK
    gobject: Box,
    // Actions
    action_tab_open: Arc<SimpleAction>,
    action_page_open: Arc<SimpleAction>,
}

impl Content {
    // Construct
    pub fn new(action_tab_open: Arc<SimpleAction>, action_page_open: Arc<SimpleAction>) -> Self {
        Self {
            gobject: Box::builder().orientation(Orientation::Vertical).build(),
            action_tab_open,
            action_page_open,
        }
    }

    // Actions
    pub fn set_status_failure(&self, title: &str, description: &str) {
        self.clean();

        let status_default = Status::new_error(title, description);

        self.gobject.append(status_default.gobject());
    }

    pub fn set_text_gemini(&self, base: &Uri, data: &str) -> Option<GString> {
        self.clean();

        let text_gemini = Text::gemini(
            data,
            base,
            self.action_tab_open.clone(),
            self.action_page_open.clone(),
        );

        self.gobject.append(text_gemini.gobject());

        text_gemini.meta_title().clone() // @TODO
    }

    pub fn clean(&self) {
        while let Some(child) = self.gobject.last_child() {
            self.gobject.remove(&child);
        }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
