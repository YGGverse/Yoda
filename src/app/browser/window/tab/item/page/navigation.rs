mod bookmark;
mod database;
mod history;
mod home;
mod reload;
mod request;
mod widget;

use bookmark::Bookmark;
use history::History;
use home::Home;
use reload::Reload;
use request::Request;
use widget::Widget;

use super::{BrowserAction, Profile, TabAction, WindowAction};
use sqlite::Transaction;
use std::rc::Rc;

pub struct Navigation {
    pub bookmark: Rc<Bookmark>,
    pub history: Rc<History>,
    pub home: Rc<Home>,
    pub profile: Rc<Profile>,
    pub reload: Rc<Reload>,
    pub request: Rc<Request>,
    pub widget: Rc<Widget>,
}

impl Navigation {
    pub fn build(
        profile: &Rc<Profile>,
        (browser_action, window_action, tab_action): (
            &Rc<BrowserAction>,
            &Rc<WindowAction>,
            &Rc<TabAction>,
        ),
    ) -> Self {
        // init children components
        let home = Rc::new(Home::build(window_action));
        let history = Rc::new(History::build(window_action));
        let reload = Rc::new(Reload::build(window_action));
        let request = Rc::new(Request::build((browser_action, tab_action)));
        let bookmark = Rc::new(Bookmark::build(window_action));

        // init main widget
        let widget = Rc::new(Widget::build(
            &home.widget.button,
            &history.widget.g_box,
            &reload.widget.button,
            &request.widget.entry,
            &bookmark.widget.button,
        ));

        // done
        Self {
            bookmark,
            history,
            home,
            profile: profile.clone(),
            reload,
            request,
            widget,
        }
    }

    // Actions

    pub fn update(&self, progress_fraction: Option<f64>) {
        // init shared request value
        let request = self.request.strip_prefix();

        // update children components
        self.bookmark
            .update(self.profile.bookmark.get(&request).is_ok());
        self.history.update();
        self.home.update(self.request.uri().as_ref());
        self.reload.update(!request.is_empty());
        self.request.update(
            progress_fraction,
            self.profile
                .identity
                .gemini
                .auth
                .memory
                .match_scope(&request)
                .is_some(),
        );
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.request.clean(transaction, &record.id)?;
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
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to the item childs
                    self.request.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match database::insert(transaction, app_browser_window_tab_item_page_id) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.request.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    request::migrate(tx)?;

    // Success
    Ok(())
}
