use gtk::prelude::WidgetExt;

pub struct Description {
    gtk: gtk::Label,
}

impl Description {
    // Construct
    pub fn new() -> Description {
        let gtk = gtk::Label::builder()
            .css_classes(["subtitle"])
            .single_line_mode(true)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .visible(false)
            .build();

        Self { gtk }
    }

    // Actions
    pub fn update(&self) {
        self.gtk.set_visible(self.gtk.text().is_empty());
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Label {
        &self.gtk
    }
}
