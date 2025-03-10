mod content;
mod database;
mod input;
mod navigation;
mod search;

use super::{Action as ItemAction, BrowserAction, Profile, TabAction, WindowAction};
use adw::TabPage;
use anyhow::Result;
use content::Content;
use input::Input;
use navigation::Navigation;
use search::Search;
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
    pub input: Rc<Input>,
    pub navigation: Rc<Navigation>,
    pub search: Rc<Search>,
    // System
    /// Reference to [TabPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabPage.html)
    /// wanted to update title, loading status and other features related with page.
    /// * this member sensitively dependent of parental HashMap index,\
    ///   as connection drivers interact with `Page` API,\
    ///   let's keep it private to isolate direct access and prevent their implementation errors
    tab_page: TabPage,
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
        tab_page: &TabPage,
    ) -> Self {
        // Init components
        let content = Rc::new(Content::build((window_action, tab_action, item_action)));
        let search = Rc::new(Search::new());
        let navigation = Rc::new(Navigation::build(
            profile,
            (window_action, tab_action, item_action),
        ));
        let input = Rc::new(Input::new());

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
            input,
            navigation,
            search,
        }
    }

    // Actions

    /// Toggle bookmark for current navigation request
    pub fn bookmark(&self) {
        self.profile
            .bookmark
            .toggle(&self.navigation.request())
            .unwrap(); // @TODO
    }

    /// Request `Escape` action for all page components
    pub fn escape(&self) {
        self.search.hide();
        self.navigation.escape();
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
    ) -> Result<()> {
        for record in database::select(transaction, app_browser_window_tab_item_id)? {
            database::delete(transaction, record.id)?;
            // Delegate clean action to the item childs
            self.navigation.clean(transaction, &record.id)?;
        }
        Ok(())
    }

    /// Restore `Self` session from database
    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<()> {
        // Begin page restore
        for record in database::select(transaction, app_browser_window_tab_item_id)? {
            // Restore `Self`
            if let Some(title) = record.title {
                self.set_title(title.as_str());
            }
            self.set_needs_attention(record.is_needs_attention);
            // Restore child components
            self.navigation.restore(transaction, &record.id)?;
            // Make initial page history snap
            self.profile.history.open(self.navigation.request());
        }
        Ok(())
    }

    /// Save `Self` session to database
    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: i64,
    ) -> Result<()> {
        // Keep value in memory until operation complete
        let title = self.tab_page.title();
        let id = database::insert(
            transaction,
            app_browser_window_tab_item_id,
            self.tab_page.needs_attention(),
            match title.is_empty() {
                true => None,
                false => Some(title.as_str()),
            },
        )?;
        // Delegate save action to childs
        self.navigation.save(transaction, &id)?;
        Ok(())
    }

    // Setters

    /// Set title for `Self`
    /// * this method allows to keep `tab_page` isolated from driver implementation
    pub fn set_title(&self, title: &str) {
        self.tab_page.set_title(title)
    }

    pub fn set_needs_attention(&self, is_needs_attention: bool) {
        self.tab_page.set_needs_attention(is_needs_attention)
    }

    pub fn set_progress(&self, progress_fraction: f64) {
        self.navigation.set_progress_fraction(progress_fraction);
        self.tab_page.set_loading(progress_fraction > 0.0)
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    navigation::migrate(tx)?;

    // Success
    Ok(())
}
