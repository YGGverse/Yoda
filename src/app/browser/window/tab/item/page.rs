mod content;
mod database;
mod error;
mod input;
mod navigation;
mod search;

use super::{Action as ItemAction, BrowserAction, Position, Profile, TabAction, WindowAction};
use adw::{TabPage, TabView};
use content::Content;
use error::Error;
use gtk::{prelude::BoxExt, Box, Orientation};
use input::Input;
use navigation::Navigation;
use search::Search;
use sourceview::prelude::IsA;
use sqlite::Transaction;
use std::rc::Rc;

pub struct Page {
    pub profile: Rc<Profile>,
    // Actions
    pub browser_action: Rc<BrowserAction>,
    pub item_action: Rc<ItemAction>,
    pub window_action: Rc<WindowAction>,
    // Components
    pub content: Rc<Content>,
    pub search: Rc<Search>,
    pub input: Rc<Input>,
    pub navigation: Rc<Navigation>,
    // System
    pub tab_page: TabPage,
}

impl Page {
    // Constructors

    pub fn build(
        profile: &Rc<Profile>,
        (browser_action, window_action, tab_action, item_action): (
            &Rc<BrowserAction>,
            &Rc<WindowAction>,
            &Rc<TabAction>,
            &Rc<ItemAction>,
        ),
        tab_view: &TabView,
        (position, is_pinned, is_selected, is_needs_attention): (Position, bool, bool, bool),
    ) -> Self {
        // Init components
        let content = Rc::new(Content::build((window_action, item_action)));
        let search = Rc::new(Search::new());
        let navigation = Rc::new(Navigation::build(
            profile,
            (window_action, tab_action, item_action),
        ));
        let input = Rc::new(Input::new());

        // Init main widget
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&navigation.g_box);
        g_box.append(&content.g_box);
        g_box.append(&search.g_box);
        g_box.append(&input.clamp);

        // Generate `TabPage` by append widget into given `TabView`
        let tab_page = match position {
            Position::After => match tab_view.selected_page() {
                Some(page) => add(tab_view, &g_box, tab_view.page_position(&page) + 1),
                None => tab_view.append(&g_box),
            },
            Position::End => tab_view.append(&g_box),
            Position::Number(value) => add(tab_view, &g_box, value),
        };

        // Setup
        tab_page.set_needs_attention(is_needs_attention);
        tab_page.set_title("New page");

        tab_view.set_page_pinned(&tab_page, is_pinned);
        if is_selected {
            tab_view.set_selected_page(&tab_page);
        }

        // Done
        Self {
            profile: profile.clone(),
            tab_page: tab_page.clone(),
            // Actions
            browser_action: browser_action.clone(),
            item_action: item_action.clone(),
            window_action: window_action.clone(),
            // Components
            content,
            search,
            input,
            navigation,
        }
    }

    // Actions

    /// Toggle bookmark for current `profile` by navigation request value
    /// * return `true` on bookmark created, `false` on deleted
    pub fn bookmark(&self) -> Result<bool, Error> {
        let result = match self
            .profile
            .bookmark
            .toggle(self.navigation.request().as_str())
        {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::Bookmark), // @TODO
        };
        result
    }

    /// Request `Escape` action for all page components
    pub fn escape(&self) {
        self.search.hide()
    }

    /// Toggle `Find` widget
    pub fn find(&self) {
        self.search.show()
    }

    /// Cleanup session for `Self`
    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.navigation.clean(transaction, &record.id)?;
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    }

    /// Restore `Self` session from database
    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<(), String> {
        // Begin page restore
        match database::select(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    // Restore main widget
                    if let Some(title) = record.title {
                        self.set_title(title.as_str());
                    }
                    // Restore self by last record
                    // Delegate restore action to the item childs
                    self.navigation.restore(transaction, &record.id)?;
                    // Make initial page history snap using `navigation` values restored
                    // * just to have back/forward navigation ability
                    if let Some(uri) = self.navigation.uri() {
                        self.profile.history.memory.request.set(uri);
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    }

    /// Save `Self` session to database
    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<(), String> {
        // Keep value in memory until operation complete
        let title = self.tab_page.title();

        match database::insert(
            transaction,
            app_browser_window_tab_item_id,
            match title.is_empty() {
                true => None,
                false => Some(title.as_str()),
            },
        ) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.navigation.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Setters

    /// Set title for `Self`
    /// * this method allows to keep `tab_page` isolated from driver implementation
    pub fn set_title(&self, title: &str) {
        self.tab_page.set_title(title);
    }

    pub fn set_progress(&self, progress_fraction: f64) {
        self.navigation.set_progress_fraction(progress_fraction);
        self.tab_page.set_loading(progress_fraction > 0.0)
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    navigation::migrate(tx)?;

    // Success
    Ok(())
}

/// Create new [TabPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabPage.html)
/// in [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) at given position
///
/// * if given `position` match pinned tab, GTK will panic with notice:
///   adw_tab_view_insert: assertion 'position >= self->n_pinned_pages'\
///   as the solution, prepend new page after pinned tabs in this case
fn add(tab_view: &TabView, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
    if position > tab_view.n_pinned_pages() {
        tab_view.insert(child, position)
    } else {
        tab_view.prepend(child)
    }
}
