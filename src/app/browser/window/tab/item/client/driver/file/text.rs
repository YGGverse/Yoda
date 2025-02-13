use super::Page;
use gtk::glib::{GString, Uri};
use std::rc::Rc;

pub enum Text {
    Gemini(Uri, String),
    Plain(Uri, String),
    Source(Uri, String),
}

impl Text {
    pub fn handle(&self, page: Rc<Page>) {
        let (uri, widget) = match self {
            Self::Gemini(uri, data) => (uri, page.content.to_text_gemini(uri, data)),
            Self::Plain(uri, data) => (uri, page.content.to_text_plain(data)),
            Self::Source(uri, data) => (uri, page.content.to_text_source(data)),
        };
        page.search.set(Some(widget.text_view));
        page.set_title(&match widget.meta.title {
            Some(title) => title.into(), // @TODO
            None => uri_to_title(uri),
        });
        page.set_progress(0.0);
        page.window_action.find.simple_action.set_enabled(true);
    }
}

/// Helper function, extract readable title from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
/// * useful as common placeholder when page title could not be detected
/// * this feature may be improved and moved outside @TODO already duplicated!
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
