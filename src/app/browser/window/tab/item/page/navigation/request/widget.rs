mod database;

use database::Database;

use gtk::{
    gio::SimpleAction,
    glib::{timeout_add_local, ControlFlow, GString, SourceId},
    prelude::{ActionExt, EditableExt, EntryExt},
    Entry,
};
use sqlite::Transaction;
use std::{cell::RefCell, sync::Arc, time::Duration};

const PLACEHOLDER_TEXT: &str = "URL or search term...";

// Progress bar animation setup
const PROGRESS_ANIMATION_STEP: f64 = 0.05;
const PROGRESS_ANIMATION_TIME: u64 = 20; //ms

struct Progress {
    fraction: RefCell<f64>,
    source_id: RefCell<Option<SourceId>>,
}

pub struct Widget {
    gobject: Entry,
    progress: Arc<Progress>,
}

impl Widget {
    // Construct
    pub fn new_arc(
        action_update: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>, // @TODO local `action_page_open`?
    ) -> Arc<Self> {
        // Init animated progress bar state
        let progress = Arc::new(Progress {
            fraction: RefCell::new(0.0),
            source_id: RefCell::new(None),
        });

        // Init widget
        let gobject = Entry::builder()
            .placeholder_text(PLACEHOLDER_TEXT)
            .hexpand(true)
            .build();

        // Connect events
        gobject.connect_changed(move |_| {
            action_update.activate(None);
        });

        gobject.connect_activate(move |_| {
            action_tab_page_navigation_reload.activate(None);
        });

        // Return activated struct
        Arc::new(Self { gobject, progress })
    }

    // Actions
    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_request_id: &i64,
    ) -> Result<(), String> {
        match Database::records(
            transaction,
            app_browser_window_tab_item_page_navigation_request_id,
        ) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            // nothing yet..
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
        app_browser_window_tab_item_page_navigation_request_id: &i64,
    ) -> Result<(), String> {
        match Database::records(
            transaction,
            app_browser_window_tab_item_page_navigation_request_id,
        ) {
            Ok(records) => {
                for record in records {
                    if let Some(text) = record.text {
                        self.gobject.set_text(&text);
                    }

                    // Delegate restore action to the item childs
                    // nothing yet..
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_request_id: &i64,
    ) -> Result<(), String> {
        // Keep value in memory until operation complete
        let text = self.gobject.text();

        match Database::add(
            transaction,
            app_browser_window_tab_item_page_navigation_request_id,
            match text.is_empty() {
                true => None,
                false => Some(text.as_str()),
            },
        ) {
            Ok(_) => {
                // let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                // nothing yet..
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn update(&self, progress_fraction: Option<f64>) {
        // Skip update animation for Non value
        if let Some(value) = progress_fraction {
            // Update shared fraction value for async progressbar function, animate on changed only
            if value != self.progress.fraction.replace(value) {
                // Start new frame on previous process function completed (`source_id` changed to None)
                // If previous process still active, we have just updated shared fraction value before, to use it inside the active process
                if self.progress.source_id.borrow().is_none() {
                    // Start new animation frame iterator, update `source_id`
                    self.progress.source_id.replace(Some(timeout_add_local(
                        Duration::from_millis(PROGRESS_ANIMATION_TIME),
                        {
                            // Clone async pointers dependency
                            let gobject = self.gobject.clone();
                            let progress = self.progress.clone();

                            // Frame
                            move || {
                                // Animate
                                if *progress.fraction.borrow() > gobject.progress_fraction() {
                                    gobject.set_progress_fraction(
                                        // Currently, here is no outrange validation, seems that wrapper make this work @TODO
                                        gobject.progress_fraction() + PROGRESS_ANIMATION_STEP,
                                    );
                                    return ControlFlow::Continue;
                                }
                                // Deactivate
                                progress.source_id.replace(None);

                                // Reset (to hide progress widget)
                                gobject.set_progress_fraction(0.0);

                                // Stop iteration
                                ControlFlow::Break
                            }
                        },
                    )));
                }
            }
        }
    }

    // Setters
    pub fn set_text(&self, value: &GString) {
        self.gobject.set_text(value);
    }

    // Getters
    pub fn gobject(&self) -> &Entry {
        &self.gobject
    }

    pub fn is_empty(&self) -> bool {
        0 == self.gobject.text_length()
    }

    pub fn text(&self) -> GString {
        self.gobject.text()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        // nothing yet..

        // Success
        Ok(())
    }
}
