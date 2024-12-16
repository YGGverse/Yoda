mod tag;

use tag::Tag;

use gtk::{prelude::TextBufferExt, TextBuffer};

pub struct Buffer {
    pub text_buffer: TextBuffer,
    pub tag: Tag,
}

impl Buffer {
    // Constructors

    /// Create new `Self`
    pub fn new(text_buffer: TextBuffer) -> Self {
        // Init components
        // * create new tag objects required for new buffer,
        //   instead of re-use existing refs (maybe the bug)
        let tag = Tag::new(text_buffer.tag_table());

        // Init `Self`
        Self { text_buffer, tag }
    }
}
