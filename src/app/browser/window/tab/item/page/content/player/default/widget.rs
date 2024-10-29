use gtk::{MediaStream, Video};

pub struct Widget {
    gobject: Video,
}

impl Widget {
    // Constructors

    /// Create new default widget configuration with options
    pub fn new(media_stream: &MediaStream) -> Self {
        Self {
            gobject: Video::builder()
                .autoplay(true)
                .hexpand(true)
                .media_stream(media_stream)
                .vexpand(true)
                .build(),
        }
    }

    // Getters

    pub fn gobject(&self) -> &Video {
        &self.gobject
    }
}
