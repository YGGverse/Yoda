pub trait Mime {
    fn mime(text: &str) -> Self;
    fn validate(&self);
}

impl Mime for gtk::Entry {
    fn mime(text: &str) -> Self {
        use gtk::prelude::EditableExt;

        let mime = gtk::Entry::builder()
            .margin_bottom(8)
            .placeholder_text("Content type (MIME)")
            .text(text)
            .build();

        mime.connect_changed(|this| {
            this.validate();
        });

        mime
    }

    fn validate(&self) {
        use gtk::prelude::{EditableExt, WidgetExt};

        const CLASS: (&str, &str) = ("error", "success");

        self.remove_css_class(CLASS.0);
        self.remove_css_class(CLASS.1);

        if !self.text().is_empty() {
            if gtk::glib::Regex::match_simple(
                r"^\w+/\w+$",
                self.text(),
                gtk::glib::RegexCompileFlags::DEFAULT,
                gtk::glib::RegexMatchFlags::DEFAULT,
            ) {
                self.add_css_class(CLASS.1)
            } else {
                self.add_css_class(CLASS.0)
            }
        }
    }
}
