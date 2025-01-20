use gtk::glib::Uri;

pub enum Text {
    Gemini { base: Uri, data: String },
    Plain { data: String },
}
