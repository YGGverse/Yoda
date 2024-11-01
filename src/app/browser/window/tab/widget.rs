use adw::TabView;
use gtk::{
    gio::{Icon, SimpleAction, SimpleActionGroup},
    glib::{uuid_string_random, GString},
    prelude::{ActionMapExt, WidgetExt},
};

pub struct Widget {
    gobject: TabView,
}

impl Widget {
    // Construct
    pub fn new(action_page_new: SimpleAction) -> Self {
        // Init additional action group
        let action_group = SimpleActionGroup::new();
        action_group.add_action(&action_page_new);

        // Init gobject
        let gobject = TabView::new();

        // Change default icon visible for tabs pinned
        if let Ok(default_icon) = Icon::for_string("view-pin-symbolic") {
            gobject.set_default_icon(&default_icon);
        }

        // Create new group for actions
        gobject.insert_action_group(&uuid_string_random(), Some(&action_group));

        Self { gobject }
    }

    // Actions
    pub fn close(&self) {
        if let Some(selected_page) = self.gobject.selected_page() {
            self.gobject.close_page(&selected_page);
        }
    }

    pub fn close_all(&self) {
        // @TODO skip pinned or make confirmation alert (GTK>=4.10)
        if let Some(selected_page) = self.gobject.selected_page() {
            self.gobject.close_other_pages(&selected_page);
            self.close();
        }
    }

    // Getters
    pub fn current_page_keyword(&self) -> Option<GString> {
        let page = self.gobject.selected_page()?;
        let id = page.keyword()?;
        Some(id)
    }

    pub fn gobject(&self) -> &TabView {
        &self.gobject
    }
}
