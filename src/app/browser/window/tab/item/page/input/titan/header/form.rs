mod mime;
mod token;

use mime::Mime;
use token::Token;

use gtk::{
    glib::GString,
    prelude::{BoxExt, EditableExt},
    Box, Entry, Orientation,
};

pub struct Form {
    pub g_box: Box,
    mime: Entry,
    token: Entry,
}

impl Form {
    // Constructors

    pub fn build(mime_value: &str, token_value: &str) -> Self {
        // Init components
        let mime = Entry::mime(mime_value);
        let token = Entry::token(token_value);

        // Init `Self`
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&mime);
        g_box.append(&token);

        Self { g_box, mime, token }
    }

    // Getters

    pub fn mime(&self) -> Option<GString> {
        value(&self.mime)
    }

    pub fn token(&self) -> Option<GString> {
        value(&self.token)
    }
}

// Tools

fn value(label: &Entry) -> Option<GString> {
    let text = label.text();
    if !text.is_empty() {
        Some(text)
    } else {
        None
    }
}
