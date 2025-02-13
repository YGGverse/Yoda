use super::{Feature, Page};
use gtk::{gio::Cancellable, glib::Uri};
use std::rc::Rc;

/// Local files client driver
pub struct File {
    page: Rc<Page>,
}

impl File {
    // Constructors

    /// Create new `Self`
    pub fn init(page: &Rc<Page>) -> Self {
        Self { page: page.clone() } // @TODO
    }

    pub fn handle(&self, uri: Uri, _feature: Rc<Feature>, cancellable: Cancellable) {
        use gtk::{
            gio::{File, FileQueryInfoFlags, FileType},
            glib::Priority,
            prelude::FileExtManual,
        };

        // try handle as directory
        File::for_uri(&uri.to_string()).enumerate_children_async(
            "standard::content-type",
            FileQueryInfoFlags::NONE,
            Priority::DEFAULT,
            Some(&cancellable),
            {
                let cancellable = cancellable.clone();
                let uri = uri.clone();
                let _page = self.page.clone();
                move |result| match result {
                    Ok(file_enumerator) => {
                        for entry in file_enumerator {
                            match entry {
                                Ok(file_info) => match file_info.file_type() {
                                    FileType::Unknown => todo!(),
                                    FileType::Regular => todo!(),
                                    FileType::Directory => todo!(),
                                    FileType::SymbolicLink => todo!(),
                                    FileType::Special => todo!(),
                                    FileType::Shortcut => todo!(),
                                    FileType::Mountable => todo!(),
                                    _ => todo!(),
                                },
                                Err(_) => todo!(),
                            }
                        }
                    }
                    // is not a directory, try handle as file
                    Err(_) => File::for_uri(&uri.to_string()).load_contents_async(
                        Some(&cancellable),
                        |result| match result {
                            Ok(_) => todo!(),
                            Err(_) => todo!(),
                        },
                    ),
                }
            },
        )
    }
}
