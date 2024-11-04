use adw::{TabPage, TabView};
use gtk::{
    gio::{Icon, MenuModel},
    prelude::IsA,
};

/// Currently used as the indicator for pinned tabs
const DEFAULT_TAB_ICON: &str = "view-pin-symbolic";

/// Wrapper for [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) GObject
pub struct Widget {
    gobject: TabView,
}

impl Widget {
    // Constructors

    /// Create new `Self`
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
    /// * this action includes `pinned` pages, to prevent that:
    ///   * deactivate [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) outside if selected page should not be closed
    ///   * use native [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) API with `GObject` reference getter
    pub fn close(&self, position: Option<i32>) {
        if let Some(page) = self.page(position) {
            self.gobject.set_page_pinned(&page, false);
            self.gobject.close_page(&page);
        }
    }

    /// Close all pages
    /// * this action includes `pinned` pages, to prevent that:
    ///   * deactivate [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) outside if selected page should not be closed
    ///   * use native [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) API with `GObject` reference getter
    pub fn close_all(&self) {
        while let Some(page) = self.gobject.selected_page() {
            self.gobject.set_page_pinned(&page, false);
            self.gobject.close_page(&page);
        }
    }

    /// Toggle pin for page at given `position`, `None` to pin selected page (if available)
    pub fn pin(&self, position: Option<i32>) {
        if let Some(page) = self.page(position) {
            self.gobject.set_page_pinned(&page, !page.is_pinned()); // toggle
        }
    }

    // Getters

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
