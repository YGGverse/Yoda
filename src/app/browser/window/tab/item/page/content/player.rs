mod default;
use default::Default;

use gtk::{MediaStream, Video};

pub struct Player {
    gobject: Video, // @TODO
}

impl Player {
    // Constructors

    pub fn new_default(media_stream: &MediaStream) -> Self {
        Self {
            gobject: Default::new(media_stream).gobject().clone(),
        }
    }

    // Getters

    pub fn gobject(&self) -> &Video {
        &self.gobject
    }
}
