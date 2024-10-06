use gtk::{glib::GString, prelude::WidgetExt, Notebook};

pub struct Widget {
    gobject: Notebook,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        let gobject = Notebook::builder().scrollable(true).build();

        Self { gobject }
    }

    // Actions
    pub fn close(&self) {
        self.gobject.remove_page(self.gobject().current_page());
    }

    pub fn close_all(&self) {
        // @TODO skip pinned or make confirmation alert (GTK>=4.10)
        while let Some(page_number) = self.gobject.current_page() {
            self.gobject.remove_page(Some(page_number));
        }
    }

    // Getters
    pub fn current_name(&self) -> Option<GString> {
        let page_number = self.gobject.current_page()?;
        let nth_page = self.gobject.nth_page(Some(page_number))?;

        let widget_name = nth_page.widget_name();
        if !widget_name.is_empty() {
            Some(widget_name)
        } else {
            None
        }
    }

    pub fn gobject(&self) -> &Notebook {
        &self.gobject
    }
}
