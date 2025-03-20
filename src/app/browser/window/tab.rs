mod action;
mod database;
mod item;
mod menu;

use super::{Action as WindowAction, Position};
use crate::Profile;
use action::Action;
use adw::{TabPage, TabView};
use anyhow::Result;
use gtk::{Box, Orientation, gio::Icon, glib::Propagation, prelude::ActionExt};
pub use item::Item;
use menu::Menu;
use sourceview::prelude::IsA;
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

// Main
pub struct Tab {
    window_action: Rc<WindowAction>,
    profile: Arc<Profile>,
    index: Rc<RefCell<HashMap<TabPage, Rc<Item>>>>,
    pub action: Rc<Action>,
    pub tab_view: TabView,
}

impl Tab {
    // Constructors

    /// Build new `Self`
    pub fn build(profile: &Arc<Profile>, window_action: &Rc<WindowAction>) -> Self {
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
        tab_view.connect_setup_menu({
            let index = index.clone();
            let window_action = window_action.clone();
            move |tab_view, tab_page| {
                // by documentation:
                // * `tab_page` == `Some` - popover open
                // * `tab_page` == `None` - popover closed
                update_actions(
                    tab_view,
                    match tab_page {
                        Some(this) => Some(this.clone()),
                        None => tab_view.selected_page(),
                    }
                    .as_ref(),
                    &index,
                    &window_action,
                );
            }
        });

        tab_view.connect_close_page({
            let index = index.clone();
            let profile = profile.clone();
            let window_action = window_action.clone();
            move |tab_view, tab_page| {
                // remove closed Item from Tab index
                if let Some(item) = index.borrow_mut().remove(tab_page) {
                    // keep removed `Item` reference in the memory (to reopen from the main menu)
                    // * skip item with blank request
                    if !item.page.navigation.request().is_empty() {
                        profile.history.close(&item.page.navigation.request());
                    }
                }
                // reassign global actions to active tab
                update_actions(
                    tab_view,
                    tab_view.selected_page().as_ref(),
                    &index,
                    &window_action,
                );
                Propagation::Proceed
            }
        });

        tab_view.connect_page_reordered({
            let window_action = window_action.clone();
            let index = index.clone();
            move |tab_view, tab_page, _| {
                update_actions(tab_view, Some(tab_page), &index, &window_action)
            }
        });

        tab_view.connect_selected_page_notify({
            let window_action = window_action.clone();
            let index = index.clone();
            move |tab_view| {
                update_actions(
                    tab_view,
                    match tab_view.selected_page() {
                        Some(this) => {
                            this.set_needs_attention(false);
                            Some(this.clone())
                        }
                        None => None,
                    }
                    .as_ref(),
                    &index,
                    &window_action,
                )
            }
        });

        // Return activated `Self`
        Self {
            profile: profile.clone(),
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
            (&self.window_action, &self.action),
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

        // Relate with GTK `TabPage` with app `Item`
        self.index
            .borrow_mut()
            .insert(item.tab_page.clone(), item.clone());

        // Setup
        self.tab_view.set_page_pinned(&item.tab_page, is_pinned);
        if is_selected {
            self.tab_view.set_selected_page(&item.tab_page);
        }

        // Forcefully update global actions on HashMap index build complete
        // * `selected_page_notify` runs this action also, just before Item init @TODO
        update_actions(
            &self.tab_view,
            self.tab_view.selected_page().as_ref(),
            &self.index,
            &self.window_action,
        );

        item
    }

    /// Close page at given `position`, `None` to close selected page (if available)
    /// * this action includes `pinned` pages, to prevent that:
    ///   * deactivate [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) outside if selected page should not be closed
    ///   * use native [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) API with `GObject` reference getter
    pub fn close(&self, page_position: Option<i32>) {
        if let Some(page) = match page_position {
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

    // Toggle search widget
    pub fn find(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.find();
        }
    }

    // Save page at given `position`, `None` to save selected page (if available)
    pub fn save_as(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.navigation.to_download();
            self.window_action.reload.activate();
        }
    }

    // View source for page at given `position`, `None` to use selected page (if available)
    pub fn source(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.navigation.to_source();
            self.window_action.reload.activate();
        }
    }

    /// Toggle `Bookmark` for `Page` at given `position` (current page on `None`)
    pub fn bookmark(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.bookmark()
        }
    }

    /// Toggle pin for page at given `position`, `None` to pin selected page (if available)
    pub fn pin(&self, page_position: Option<i32>) {
        if let Some(page) = match page_position {
            Some(value) => Some(self.tab_view.nth_page(value)),
            None => self.tab_view.selected_page(),
        } {
            self.tab_view.set_page_pinned(&page, !page.is_pinned()); // toggle
        }
    }

    pub fn home(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.action.home.activate(None)
        }
    }

    pub fn history_back(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.action.history.back.activate(None)
        }
    }

    pub fn history_forward(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.action.history.forward.activate(None)
        }
    }

    /// Reload page at `i32` position or selected page on `None` given
    pub fn reload(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.client.handle(&item.page.navigation.request(), true);
        }
    }

    pub fn open(&self, page_position: Option<i32>, request: &str) {
        if let Some(item) = self.item(page_position) {
            item.action.load.activate(Some(request), true);
        }
    }

    pub fn clean(&self, transaction: &Transaction, app_browser_window_id: i64) -> Result<()> {
        for record in database::select(transaction, app_browser_window_id)? {
            database::delete(transaction, record.id)?;
            // Delegate clean action to childs
            for (_, item) in self.index.borrow().iter() {
                item.clean(transaction, record.id)?
            }
        }
        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_browser_window_id: i64) -> Result<()> {
        for tab_record in database::select(transaction, app_browser_window_id)? {
            for item_record in item::restore(transaction, tab_record.id)? {
                // Generate new `TabPage` with blank `Widget`
                let (tab_page, target_child) =
                    new_tab_page(&self.tab_view, Position::Number(item_record.page_position));

                // Init new tab item
                let item = Rc::new(Item::build(
                    (&tab_page, &target_child),
                    &self.profile,
                    // Actions
                    (&self.window_action, &self.action),
                    // Options
                    None,
                    false,
                ));

                // Relate with GTK `TabPage` with app `Item`
                self.index
                    .borrow_mut()
                    .insert(item.tab_page.clone(), item.clone());

                // Setup
                self.tab_view
                    .set_page_pinned(&item.tab_page, item_record.is_pinned);

                if item_record.is_selected {
                    self.tab_view.set_selected_page(&item.tab_page);
                }

                // Forcefully update global actions on HashMap index build complete
                // * `selected_page_notify` runs this action also, just before Item init @TODO
                update_actions(
                    &self.tab_view,
                    self.tab_view.selected_page().as_ref(),
                    &self.index,
                    &self.window_action,
                );

                // Restore children components
                item.page.restore(transaction, item_record.id)?;
            }
        }
        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_browser_window_id: i64) -> Result<()> {
        let id = database::insert(transaction, app_browser_window_id)?;
        for (_, item) in self.index.borrow().iter() {
            item.save(transaction, id, self.tab_view.page_position(&item.tab_page))?;
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

    /// Find `Item` by `TabPage` position in HashMap `index`
    fn item(&self, page_position: Option<i32>) -> Option<Rc<Item>> {
        match page_position {
            Some(value) => Some(self.tab_view.nth_page(value)),
            None => self.tab_view.selected_page(),
        }
        .map(|tab_page| self.index.borrow().get(&tab_page).unwrap().clone())
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    item::migrate(tx)?;

    // Success
    Ok(())
}

/// Update global actions for given [TabPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabPage.html)
/// using `Item` match relation in the HashMap `index`
fn update_actions(
    tab_view: &TabView,
    tab_page: Option<&TabPage>,
    index: &Rc<RefCell<HashMap<TabPage, Rc<Item>>>>,
    window_action: &Rc<WindowAction>,
) {
    if let Some(tab_page) = tab_page {
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
            window_action
                .save_as
                .simple_action
                .set_enabled(!item.page.navigation.is_file());

            window_action.change_state(Some(tab_view.page_position(tab_page)));
            return;
        } // @TODO `connect_selected_page_notify` panics on unwrap
    }
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

/// Create new [TabPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabPage.html)
/// in [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html)
/// with given child Widget at given position
///
/// * if the `position` match pinned tab, GTK will panic with notice:
///   adw_tab_view_insert: assertion 'position >= self->n_pinned_pages'\
///   this shared method prepends new page after pinned tabs as the solution
fn add_tab_page(tab_view: &TabView, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
    if position > tab_view.n_pinned_pages() {
        tab_view.insert(child, position)
    } else {
        tab_view.prepend(child)
    }
}

/// Create new [TabPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabPage.html)
/// in [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) at app `Position`
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
