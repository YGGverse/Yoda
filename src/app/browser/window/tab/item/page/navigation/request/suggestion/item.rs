mod imp;

use gtk::glib::{self, GString, Object};

glib::wrapper! {
    pub struct Item(ObjectSubclass<imp::Item>);
}

impl Item {
    // Constructors

    pub fn build(title: GString, subtitle: GString, has_bookmark: bool, request: GString) -> Self {
        Object::builder()
            .property("title", title)
            .property("subtitle", subtitle)
            .property("request", request)
            .property("has-bookmark", has_bookmark)
            .build()
    }
}
