use super::Page;
use gtk::{gdk::Texture, glib::Uri};
use std::rc::Rc;

pub enum Image {
    Bitmap(Uri, Texture),
    // @TODO Vector(Uri, String),
}

impl Image {
    pub fn handle(&self, page: Rc<Page>) {
        let uri = match self {
            Self::Bitmap(uri, texture) => {
                page.content.to_image(texture);
                uri
            }
        };
        page.set_title(&crate::tool::uri_to_title(uri));
        page.set_progress(0.0);
        page.window_action.find.simple_action.set_enabled(false);
    }
}
