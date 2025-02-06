pub trait Mime {
    fn mime(text: &str) -> Self;
    fn validate(&self);
}

impl Mime for gtk::Entry {
    fn mime(text: &str) -> Self {
        use gtk::prelude::{EditableExt, WidgetExt};
        use std::time::Duration;

        const TEXT: &str = "Content type (MIME)";

        let mime = gtk::Entry::builder()
            .margin_bottom(8)
            .placeholder_text(TEXT)
            .text(text)
            //.tooltip_text(TEXT)
            .build();

        mime.connect_realize(|this| {
            gtk::glib::timeout_add_local_once(Duration::from_millis(100), {
                let this = this.clone();
                move || {
                    this.select_region(0, 0);
                }
            }); // remove auto-selection for the first child @TODO unstable
        });
        mime.connect_changed(|this| this.validate());
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
