mod content;
mod database;
mod error;
mod input;
mod navigation;
mod notice;
mod search;

use super::{Action as ItemAction, BrowserAction, Profile, TabAction, WindowAction};
use adw::{Banner, TabPage};
use content::Content;
use error::Error;
use input::Input;
use navigation::Navigation;
use notice::Notice;
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
    pub notice: Banner,
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
        let notice = Banner::notice();

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
            notice,
            search,
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

    /// Toggle `Notice` widget
    pub fn notice(&self, title: &str) {
        self.notice.show(title)
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
                    // Restore `Self`
                    if let Some(title) = record.title {
                        self.set_title(title.as_str());
                    }
                    self.set_needs_attention(record.is_needs_attention);
                    // Restore child components
                    self.navigation.restore(transaction, &record.id)?;
                    // Make initial page history snap using `navigation` values restored
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
            self.tab_page.needs_attention(),
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
