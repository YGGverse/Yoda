use gtk::glib::Uri;

pub enum Text {
    Gemini(Uri, String),
    Plain(Uri, String),
    Source(Uri, String),
}

impl Text {
    pub fn handle(&self, page: &super::Page) {
        let (uri, widget) = match self {
            Self::Gemini(uri, data) => (uri, page.content.to_text_gemini(uri, data)),
            Self::Plain(uri, data) => (uri, page.content.to_text_plain(data)),
            Self::Source(uri, data) => (uri, page.content.to_text_source(data)),
        };
        page.search.set(Some(widget.text_view));
        page.set_title(&match widget.meta.title {
            Some(title) => title.into(), // @TODO
            None => crate::tool::uri_to_title(uri),
        });
        page.set_progress(0.0);
        page.window_action.find.simple_action.set_enabled(true);
    }
}
