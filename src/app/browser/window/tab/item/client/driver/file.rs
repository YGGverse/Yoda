use super::{Feature, Page};
use gtk::{gio::Cancellable, glib::Uri};
use std::rc::Rc;

/// Local files client driver
pub struct File {
    // page: Rc<Page>,
}

impl File {
    // Constructors

    /// Create new `Self`
    pub fn init(_page: &Rc<Page>) -> Self {
        Self { /*page: page.clone()*/ } // @TODO
    }

    pub fn handle(&self, uri: Uri, _feature: Rc<Feature>, cancellable: Cancellable) {
        use gtk::{gio::File, prelude::FileExtManual};

        File::for_uri(&uri.to_string()).load_contents_async(Some(&cancellable), |result| {
            match result {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        });
    }
}
