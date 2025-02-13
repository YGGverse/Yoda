mod image;
mod status;
mod text;

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

    pub fn handle(&self, uri: Uri, feature: Rc<Feature>, cancellable: Cancellable) {
        use gtk::{
            gio::{File, FileQueryInfoFlags, FileType},
            glib::Priority,
            prelude::{FileExt, FileExtManual},
        };
        use image::Image;
        use status::Status;
        use text::Text;

        let url = uri.to_string();
        let file = File::for_uri(&url);
        let page = self.page.clone();

        match file.query_file_type(FileQueryInfoFlags::NONE, Some(&cancellable)) {
            FileType::Directory => file.enumerate_children_async(
                "standard::content-type",
                FileQueryInfoFlags::NONE,
                Priority::DEFAULT,
                Some(&cancellable),
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
                                Err(e) => Status::Failure(e.to_string()).handle(&page),
                            }
                        }
                    }
                    Err(e) => Status::Failure(e.to_string()).handle(&page),
                },
            ),
            _ => file.clone().query_info_async(
                "standard::content-type",
                FileQueryInfoFlags::NONE,
                Priority::DEFAULT,
                Some(&cancellable.clone()),
                move |result| match result {
                    Ok(file_info) => match file_info.content_type() {
                        Some(content_type) => match content_type.as_str() {
                            "text/plain" => {
                                if matches!(*feature, Feature::Source) {
                                    load_contents_async(file, cancellable, move |result| {
                                        match result {
                                            Ok(data) => Text::Source(uri, data).handle(&page),
                                            Err(message) => Status::Failure(message).handle(&page),
                                        }
                                    })
                                } else if url.ends_with(".txt") {
                                    load_contents_async(file, cancellable, move |result| {
                                        match result {
                                            Ok(data) => Text::Plain(uri, data).handle(&page),
                                            Err(message) => Status::Failure(message).handle(&page),
                                        }
                                    });
                                } else {
                                    load_contents_async(file, cancellable, move |result| {
                                        match result {
                                            Ok(data) => Text::Gemini(uri, data).handle(&page),
                                            Err(message) => Status::Failure(message).handle(&page),
                                        }
                                    })
                                }
                            }
                            "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                                match gtk::gdk::Texture::from_file(&file) {
                                    Ok(texture) => Image::Bitmap(uri, texture).handle(&page),
                                    Err(e) => Status::Failure(e.to_string()).handle(&page),
                                }
                            }
                            mime => {
                                Status::Failure(format!("Content type `{mime}` yet not supported"))
                                    .handle(&page)
                            }
                        },
                        None => {
                            Status::Failure("Undetectable content type".to_string()).handle(&page)
                        }
                    },
                    Err(e) => Status::Failure(e.to_string()).handle(&page),
                },
            ),
        }
    }
}

// Tools

fn load_contents_async(
    file: gtk::gio::File,
    cancellable: Cancellable,
    callback: impl FnOnce(Result<String, String>) + 'static,
) {
    use gtk::prelude::FileExtManual;
    file.load_contents_async(Some(&cancellable), {
        move |result| {
            callback(match result {
                Ok((ref buffer, _)) => match String::from_utf8(buffer.to_vec()) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(e.to_string()),
                },
                Err(e) => Err(e.to_string()),
            })
        }
    })
}
