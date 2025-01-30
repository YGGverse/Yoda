mod action;
mod database;
mod error;
mod item;
mod menu;

use super::{Action as WindowAction, BrowserAction, Position};
use crate::Profile;
use action::Action;
use adw::{TabPage, TabView};
use error::Error;
use gtk::{
    gio::Icon,
    glib::{DateTime, Propagation},
    prelude::ActionExt,
    Box, Orientation,
};
pub use item::Item;
use menu::Menu;
use sourceview::prelude::IsA;
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Main
pub struct Tab {
    browser_action: Rc<BrowserAction>,
    window_action: Rc<WindowAction>,
    profile: Rc<Profile>,
    index: Rc<RefCell<HashMap<TabPage, Rc<Item>>>>,
    pub action: Rc<Action>,
    pub tab_view: TabView,
}

impl Tab {
    // Constructors

    /// Build new `Self`
    pub fn build(
        profile: &Rc<Profile>,
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
    ) -> Self {
        let action = Rc::new(Action::new());

        // Init empty HashMap index
        let index = Rc::new(RefCell::new(HashMap::new()));

        // Init model
        let tab_view = TabView::builder()
            .menu_model(&gtk::gio::Menu::menu(window_action))
            .build();

        // Change default icon (if available in the system icon set)
        // * visible for pinned tabs only
        // * @TODO not default GTK behavior, make this feature optional
        if let Ok(default_icon) = Icon::for_string("view-pin-symbolic") {
            tab_view.set_default_icon(&default_icon);
        }

        // Init events
        tab_view.connect_close_page({
            let index = index.clone();
            let profile = profile.clone();
            let window_action = window_action.clone();
            move |tab_view, tab_page| {
                // cleanup HashMap index
                // add history record into profile memory pool
                // * this action allows to recover recently closed tab (e.g. from the main menu)
                profile.history.memory.tab.add(
                    index.borrow_mut().remove(tab_page).unwrap(),
                    DateTime::now_local().unwrap().to_unix(),
                );
                update_actions(
                    tab_view,
                    tab_view.selected_page().as_ref(),
                    &index,
                    &window_action,
                );
                Propagation::Proceed
            }
        });

        tab_view.connect_selected_page_notify({
            let window_action = window_action.clone();
            let index = index.clone();
            move |tab_view| {
                if let Some(tab_page) = tab_view.selected_page() {
                    tab_page.set_needs_attention(false);
                }
                update_actions(
                    tab_view,
                    tab_view.selected_page().as_ref(),
                    &index,
                    &window_action,
                )
            }
        });

        // Return activated `Self`
        Self {
            profile: profile.clone(),
            browser_action: browser_action.clone(),
            window_action: window_action.clone(),
            index,
            tab_view,
            action,
        }
    }

    // Actions
    pub fn append(
        &self,
        position: Position,
        request: Option<&str>,
        is_pinned: bool,
        is_selected: bool,
        is_needs_attention: bool,
        is_load: bool,
    ) -> Rc<Item> {
        // Generate new `TabPage` with blank `Widget`
        let (tab_page, target_child) = new_tab_page(&self.tab_view, position);

        // Init new tab item
        let item = Rc::new(Item::build(
            (&tab_page, &target_child),
            &self.profile,
            // Actions
            (&self.browser_action, &self.window_action, &self.action),
            // Options
            request,
            is_load,
        ));

        // Make initial setup
        item.page.set_needs_attention(is_needs_attention);
        item.page.set_title("New page");

        // Expect user input on tab appended has empty request entry
        // * this action initiated here because should be applied on tab appending event only
        if request.is_none() || request.is_some_and(|value| value.is_empty()) {
            item.page.navigation.grab_focus();
        }

        // Register dynamically created tab components in the HashMap index
        self.index
            .borrow_mut()
            .insert(item.tab_page.clone(), item.clone());

        // Setup
        // * important to call these actions after index!
        self.tab_view.set_page_pinned(&item.tab_page, is_pinned);
        if is_selected {
            self.tab_view.set_selected_page(&item.tab_page);
        }

        item
    }

    /// Close page at given `position`, `None` to close selected page (if available)
    /// * this action includes `pinned` pages, to prevent that:
    ///   * deactivate [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) outside if selected page should not be closed
    ///   * use native [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) API with `GObject` reference getter
    pub fn close(&self, tab_page_position: Option<i32>) {
        if let Some(page) = match tab_page_position {
            Some(value) => Some(self.tab_view.nth_page(value)),
            None => self.tab_view.selected_page(),
        } {
            self.tab_view.set_page_pinned(&page, false);
            self.tab_view.close_page(&page);
        }
    }

    /// Close all pages
    /// * this action includes `pinned` pages, to prevent that:
    ///   * deactivate [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) outside if selected page should not be closed
    ///   * use native [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) API with `GObject` reference getter
    pub fn close_all(&self) {
        while let Some(page) = self.tab_view.selected_page() {
            self.tab_view.set_page_pinned(&page, false);
            self.tab_view.close_page(&page);
        }
    }

    // Toggle escape action for specified or current item
    pub fn escape(&self) {
        if let Some(item) = self.item(None) {
            item.page.escape();
        }
    }

    // Toggle search widget
    pub fn find(&self, tab_page_position: Option<i32>) {
        if let Some(item) = self.item(tab_page_position) {
            item.page.find();
        }
    }

    // Save page at given `position`, `None` to save selected page (if available)
    pub fn save_as(&self, tab_page_position: Option<i32>) {
        if let Some(item) = self.item(tab_page_position) {
            item.page.navigation.to_download();
            self.window_action.reload.activate();
        }
    }

    // View source for page at given `position`, `None` to use selected page (if available)
    pub fn source(&self, tab_page_position: Option<i32>) {
        if let Some(item) = self.item(tab_page_position) {
            item.page.navigation.to_source();
            self.window_action.reload.activate();
        }
    }

    /// Toggle `Bookmark` in current `Profile` for `Page` at given `position` (current page on `None`)
    /// * return `true` on bookmark created, `false` on deleted; `Error` otherwise.
    pub fn bookmark(&self, tab_page_position: Option<i32>) -> Result<bool, Error> {
        if let Some(item) = self.item(tab_page_position) {
            return match item.page.bookmark() {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::Bookmark),
            };
        }
        Err(Error::PageNotFound)
    }

    /// Toggle pin for page at given `position`, `None` to pin selected page (if available)
    pub fn pin(&self, tab_page_position: Option<i32>) {
        if let Some(page) = match tab_page_position {
            Some(value) => Some(self.tab_view.nth_page(value)),
            None => self.tab_view.selected_page(),
        } {
            self.tab_view.set_page_pinned(&page, !page.is_pinned()); // toggle
        }
    }

    pub fn page_home(&self, tab_page_position: Option<i32>) {
        if let Some(item) = self.item(tab_page_position) {
            if let Some(home) = item.page.navigation.home() {
                let home = home.to_string();
                item.page.navigation.set_request(&home);
                item.client.handle(&home, true);
            }
        }
    }

    pub fn page_history_back(&self, tab_page_position: Option<i32>) {
        if let Some(item) = self.item(tab_page_position) {
            item.action.history.back(true);
        }
    }

    pub fn page_history_forward(&self, tab_page_position: Option<i32>) {
        if let Some(item) = self.item(tab_page_position) {
            item.action.history.forward(true);
        }
    }

    /// Reload page at `i32` position or selected page on `None` given
    pub fn page_reload(&self, tab_page_position: Option<i32>) {
        if let Some(item) = self.item(tab_page_position) {
            item.client.handle(&item.page.navigation.request(), true);
        }
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_id: i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            for (_, item) in self.index.borrow().iter() {
                                item.clean(transaction, record.id)?
                            }
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    }

    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_id: i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_id) {
            Ok(tab_records) => {
                for tab_record in tab_records {
                    for item_record in item::restore(transaction, tab_record.id)? {
                        // Generate new `TabPage` with blank `Widget`
                        let (tab_page, target_child) =
                            new_tab_page(&self.tab_view, Position::After);

                        // Init new tab item
                        let item = Rc::new(Item::build(
                            (&tab_page, &target_child),
                            &self.profile,
                            // Actions
                            (&self.browser_action, &self.window_action, &self.action),
                            // Options
                            None,
                            false,
                        ));

                        self.index
                            .borrow_mut()
                            .insert(item.tab_page.clone(), item.clone());

                        // Restore `Self`
                        // * important to call these actions after index!
                        self.tab_view
                            .set_page_pinned(&item.tab_page, item_record.is_pinned);

                        if item_record.is_selected {
                            self.tab_view.set_selected_page(&item.tab_page);
                        }

                        // Restore children components
                        item.page.restore(transaction, item_record.id)?;
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_id: i64,
    ) -> Result<(), String> {
        match database::insert(transaction, app_browser_window_id) {
            Ok(_) => {
                // Delegate save action to childs
                let id = database::last_insert_id(transaction);
                for (_, item) in self.index.borrow().iter() {
                    item.save(transaction, id, self.tab_view.page_position(&item.tab_page))?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    }

    pub fn init(&self) {
        // Append just one blank page if no tabs available after last session restore
        if self.index.borrow().is_empty() {
            self.append(Position::End, None, false, true, false, false);
        }

        // @TODO other/child features..
    }

    fn item(&self, page_position: Option<i32>) -> Option<Rc<Item>> {
        if let Some(tab_page) = match page_position {
            Some(value) => Some(self.tab_view.nth_page(value)),
            None => self.tab_view.selected_page(),
        } {
            if let Some(item) = self.index.borrow().get(&tab_page) {
                return Some(item.clone());
            }
        }
        None
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    item::migrate(tx)?;

    // Success
    Ok(())
}

fn update_actions(
    tab_view: &TabView,
    tab_page: Option<&TabPage>,
    index: &Rc<RefCell<HashMap<TabPage, Rc<Item>>>>,
    window_action: &Rc<WindowAction>,
) {
    match tab_page {
        Some(tab_page) => {
            if let Some(item) = index.borrow().get(tab_page) {
                window_action
                    .home
                    .simple_action
                    .set_enabled(item.action.home.is_enabled());
                window_action
                    .reload
                    .simple_action
                    .set_enabled(item.action.reload.is_enabled());
                window_action
                    .history_back
                    .simple_action
                    .set_enabled(item.action.history.back.is_enabled());
                window_action
                    .history_forward
                    .simple_action
                    .set_enabled(item.action.history.forward.is_enabled());

                window_action.change_state(Some(tab_view.page_position(tab_page)));
            } // @TODO incorrect index init implementation, tabs refactory wanted
        }
        None => {
            // Reset to defaults
            window_action.home.simple_action.set_enabled(false);
            window_action.reload.simple_action.set_enabled(false);
            window_action.history_back.simple_action.set_enabled(false);
            window_action
                .history_forward
                .simple_action
                .set_enabled(false);

            window_action.change_state(None);
        }
    }
}

/// Create new [TabPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabPage.html)
/// in [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) at given position
///
/// * if given `position` match pinned tab, GTK will panic with notice:
///   adw_tab_view_insert: assertion 'position >= self->n_pinned_pages'\
///   as the solution, prepend new page after pinned tabs in this case
fn add_tab_page(tab_view: &TabView, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
    if position > tab_view.n_pinned_pages() {
        tab_view.insert(child, position)
    } else {
        tab_view.prepend(child)
    }
}
fn new_tab_page(tab_view: &TabView, position: Position) -> (TabPage, Box) {
    let child = Box::builder().orientation(Orientation::Vertical).build();
    (
        match position {
            Position::After => match tab_view.selected_page() {
                Some(selected_page) => {
                    add_tab_page(tab_view, &child, tab_view.page_position(&selected_page) + 1)
                }
                None => tab_view.append(&child),
            },
            Position::End => tab_view.append(&child),
            Position::Number(value) => add_tab_page(tab_view, &child, value),
        },
        child,
    )
}
