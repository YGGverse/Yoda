use super::{Feature, Page};
use gtk::{
    gio::Cancellable,
    glib::{GString, Uri},
};
use sourceview::prelude::FileExt;
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
            prelude::FileExtManual,
        };

        let url = uri.to_string();

        match File::for_uri(&url).query_file_type(FileQueryInfoFlags::NONE, Some(&cancellable)) {
            FileType::Directory => File::for_uri(&url).enumerate_children_async(
                "standard::content-type",
                FileQueryInfoFlags::NONE,
                Priority::DEFAULT,
                Some(&cancellable),
                |result| match result {
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
                    Err(_) => todo!(),
                },
            ),
            _ => {
                if url.ends_with(".gmi") || url.ends_with(".gemini") {
                    if matches!(*feature, Feature::Source) {
                        text_source(&self.page, uri, cancellable)
                    } else {
                        text_gemini(&self.page, uri, cancellable)
                    }
                } else if url.ends_with(".txt") {
                    text_plain(&self.page, uri, cancellable)
                } else if !url.ends_with("/") {
                    text_gemini(&self.page, uri, cancellable)
                } else {
                    status_failure(&self.page, "Unsupported content type")
                }
            }
        }
    }
}

// Tools

/// Handle as `text/source`
fn text_source(page: &Rc<Page>, uri: Uri, cancellable: Cancellable) {
    load_contents_async(&uri.to_string(), cancellable, {
        let page = page.clone();
        let uri = uri.clone();
        move |data| text(page, uri, Text::Source(data))
    });
}

/// Handle as `text/gemini`
fn text_gemini(page: &Rc<Page>, uri: Uri, cancellable: Cancellable) {
    load_contents_async(&uri.to_string(), cancellable, {
        let page = page.clone();
        let uri = uri.clone();
        move |data| text(page, uri, Text::Gemini(data))
    });
}

/// Handle as `text/plain`
fn text_plain(page: &Rc<Page>, uri: Uri, cancellable: Cancellable) {
    load_contents_async(&uri.to_string(), cancellable, {
        let page = page.clone();
        let uri = uri.clone();
        move |data| text(page, uri, Text::Plain(data))
    });
}

/// Handle as failure status page
fn status_failure(page: &Rc<Page>, message: &str) {
    let status = page.content.to_status_failure();
    status.set_description(Some(message));
    page.set_title(&status.title());
    page.set_progress(0.0);
}

fn load_contents_async(
    url: &str,
    cancellable: Cancellable,
    on_success: impl FnOnce(String) + 'static,
) {
    use gtk::prelude::FileExtManual;
    gtk::gio::File::for_uri(url).load_contents_async(Some(&cancellable), {
        move |result| match result {
            Ok((ref buffer, _)) => match String::from_utf8(buffer.to_vec()) {
                Ok(data) => on_success(data),
                Err(_) => todo!(),
            },
            Err(_) => todo!(),
        }
    })
}

enum Text {
    Gemini(String),
    Plain(String),
    Source(String),
}

/// Handle as text
fn text(page: Rc<Page>, uri: Uri, text: Text) {
    let widget = match text {
        Text::Gemini(data) => page.content.to_text_gemini(&uri, &data),
        Text::Plain(data) => page.content.to_text_plain(&data),
        Text::Source(data) => page.content.to_text_source(&data),
    };
    page.search.set(Some(widget.text_view));
    page.set_title(&match widget.meta.title {
        Some(title) => title.into(), // @TODO
        None => uri_to_title(&uri),
    });
    page.set_progress(0.0);
    page.window_action.find.simple_action.set_enabled(true);
}

/// Helper function, extract readable title from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
/// * useful as common placeholder when page title could not be detected
/// * this feature may be improved and moved outside @TODO
fn uri_to_title(uri: &Uri) -> GString {
    let path = uri.path();
    if path.split('/').last().unwrap_or_default().is_empty() {
        match uri.host() {
            Some(host) => host,
            None => "Untitled".into(),
        }
    } else {
        path
    }
}
