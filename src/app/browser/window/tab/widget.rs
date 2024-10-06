use gtk::{glib::GString, prelude::WidgetExt, Box, Notebook};

pub struct Widget {
    gobject: Notebook,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        Self {
            gobject: Notebook::builder().scrollable(true).build(),
        }
    }

    // Actions
    pub fn append(
        &self,
        label: &Box,
        page: &Box,
        is_current_page: bool,
        is_reorderable: bool,
    ) -> u32 {
        // Append new Notebook page
        let page_number = self.gobject.append_page(page, Some(label));

        // Additional setup for Notebook tab created
        self.gobject.set_tab_reorderable(page, is_reorderable);

        if is_current_page {
            self.gobject.set_current_page(Some(page_number));
        }

        // Result
        page_number
    }

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
