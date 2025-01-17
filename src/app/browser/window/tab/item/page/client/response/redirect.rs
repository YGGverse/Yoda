use gtk::glib::Uri;

pub enum Redirect {
    Foreground { source: Uri, target: Uri },
    Background { source: Uri, target: Uri },
}
