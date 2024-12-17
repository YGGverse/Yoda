mod tag;

use tag::Tag;

use gtk::{
    prelude::{TextBufferExt, TextViewExt},
    TextView,
};

pub struct Subject {
    pub text_view: TextView,
    pub tag: Tag,
}

impl Subject {
    // Constructors

    /// Create new `Self`
    pub fn new(text_view: TextView) -> Self {
        // Init components
        // * create new tag objects required for new buffer,
        //   instead of re-use existing refs (maybe the bug)
        let tag = Tag::new(text_view.buffer().tag_table());

        // Init `Self`
        Self { text_view, tag }
    }
}
