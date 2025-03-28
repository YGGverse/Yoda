mod directory;
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

    pub fn handle(
        &self,
        uri: Uri,
        feature: Rc<Feature>,
        cancellable: Cancellable,
        is_snap_history: bool,
    ) {
        use directory::Directory;
        use gtk::{
            gio::{File, FileQueryInfoFlags, FileType},
            glib::Priority,
            prelude::FileExt,
        };
        use image::Image;
        use status::Status;
        use text::Text;

        {
            let mut i = self.page.navigation.request.info.borrow_mut();
            i.set_request(Some(uri.to_string()));
            self.page.navigation.request.update_secondary_icon(&i)
        }

        let url = uri.to_string();
        let file = File::for_uri(&url);
        let page = self.page.clone();

        match file.query_file_type(FileQueryInfoFlags::NONE, Some(&cancellable)) {
            FileType::Directory => Directory { file }.handle(&page, is_snap_history),
            _ => file.clone().query_info_async(
                "standard::content-type,standard::size",
                FileQueryInfoFlags::NONE,
                Priority::DEFAULT,
                Some(&cancellable.clone()),
                move |result| match result {
                    Ok(file_info) => {
                        page.navigation
                            .request
                            .info
                            .borrow_mut()
                            .set_size(Some(file_info.size() as usize));
                        match file_info.content_type() {
                            Some(content_type) => {
                                {
                                    page.navigation
                                        .request
                                        .info
                                        .borrow_mut()
                                        .set_mime(Some(content_type.to_string()));
                                }
                                match content_type.as_str() {
                                    "text/plain" => {
                                        if matches!(*feature, Feature::Source) {
                                            load_contents_async(file, cancellable, move |result| {
                                                match result {
                                                    Ok(data) => {
                                                        Text::Source(uri, data).handle(&page)
                                                    }
                                                    Err(message) => {
                                                        Status::Failure(message).handle(&page)
                                                    }
                                                }
                                            })
                                        } else if url.ends_with(".txt") {
                                            load_contents_async(file, cancellable, move |result| {
                                                match result {
                                                    Ok(data) => {
                                                        Text::Plain(uri, data).handle(&page)
                                                    }
                                                    Err(message) => {
                                                        Status::Failure(message).handle(&page)
                                                    }
                                                }
                                            });
                                        } else {
                                            load_contents_async(file, cancellable, move |result| {
                                                match result {
                                                    Ok(data) => {
                                                        Text::Gemini(uri, data).handle(&page)
                                                    }
                                                    Err(message) => {
                                                        Status::Failure(message).handle(&page)
                                                    }
                                                }
                                            })
                                        }
                                    }
                                    "image/png" | "image/gif" | "image/jpeg" | "image/webp" => {
                                        match gtk::gdk::Texture::from_file(&file) {
                                            Ok(texture) => {
                                                Image::Bitmap(uri, texture).handle(&page)
                                            }
                                            Err(e) => Status::Failure(e.to_string()).handle(&page),
                                        }
                                    }
                                    mime => Status::Failure(format!(
                                        "Content type `{mime}` yet not supported"
                                    ))
                                    .handle(&page),
                                }
                            }
                            None => Status::Failure("Undetectable content type".to_string())
                                .handle(&page),
                        }
                    }
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
