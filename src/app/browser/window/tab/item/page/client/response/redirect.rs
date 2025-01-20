use gtk::glib::Uri;

pub enum Redirect {
    Foreground(Uri),
    Background(Uri),
}
