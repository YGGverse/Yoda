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
            .build();

        Self { gtk }
    }

    // Actions
    pub fn update(&self, text: &str) {
        self.gtk.set_text(text);

        if text.is_empty() {
            self.gtk.hide();
        } else {
            self.gtk.show();
        }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Label {
        &self.gtk
    }
}
