mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

impl Item {
    // Constructors

    pub fn build(title: String, subtitle: String, request: String) -> Self {
        Object::builder()
            .property("title", title)
            .property("subtitle", subtitle)
            .property("request", request)
            .build()
    }
}
