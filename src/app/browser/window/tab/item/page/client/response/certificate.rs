use gtk::glib::GString;

pub enum Certificate {
    Invalid { title: GString },
    Request { title: GString },
    Unauthorized { title: GString },
}
