use adw::{TabPage, TabView};
use gtk::{
    gio::{Icon, MenuModel},
    glib::GString,
    prelude::IsA,
};

/// Currently used as the indicator for pinned tabs
const DEFAULT_TAB_ICON: &str = "view-pin-symbolic";

/// Wrapper for [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) GObject
pub struct Widget {
    gobject: TabView,
}

impl Widget {
    // Construct
    pub fn new(menu_model: &impl IsA<MenuModel>) -> Self {
        // Init gobject
        let gobject = TabView::builder().menu_model(menu_model).build();

        // Change default icon (if available in the system icon set)
        // * visible for pinned tabs only
        // * @TODO not default GTK behavior, make this feature optional
        if let Ok(default_icon) = Icon::for_string(DEFAULT_TAB_ICON) {
            gobject.set_default_icon(&default_icon);
        }

        // Done
        Self { gobject }
    }

    // Actions

    /// Close page at given `position`, `None` to close selected page (if available)
    pub fn close(&self, position: Option<i32>) {
        if let Some(page) = self.page(position) {
            self.gobject.close_page(&page);
        }
    }

    /// Close all pages, including selected one
    pub fn close_all(&self) {
        // @TODO skip pinned or make confirmation alert (GTK>=4.10)
        if let Some(selected_page) = self.gobject.selected_page() {
            self.gobject.close_other_pages(&selected_page);
            self.close(None);
        }
    }

    // Getters

    pub fn current_page_keyword(&self) -> Option<GString> {
        let page = self.gobject.selected_page()?;
        let id = page.keyword()?;
        Some(id)
    } // @TODO remove as deprecated

    /// Get **keyword** for page at given position, `None` for selected page
    /// * return `None` if requested page or selected not found
    pub fn page_keyword(&self, position: Option<i32>) -> Option<GString> {
        self.page(position)?.keyword()
    }

    /// Get tab page by position, `None` for selected page
    /// * return `None` if requested or selected page not found
    pub fn page(&self, position: Option<i32>) -> Option<TabPage> {
        match position {
            Some(value) => Some(self.gobject.nth_page(value)),
            None => self.gobject.selected_page(),
        }
    }

    /// Get reference of [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) `GObject`
    pub fn gobject(&self) -> &TabView {
        &self.gobject
    }
}
