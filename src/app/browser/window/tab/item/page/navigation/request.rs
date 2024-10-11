mod database;

use database::Database;

use gtk::{
    gio::SimpleAction,
    glib::{timeout_add_local, ControlFlow, GString, SourceId, Uri, UriFlags},
    prelude::{ActionExt, EditableExt, EntryExt},
    Entry,
};
use sqlite::Transaction;
use std::{cell::RefCell, sync::Arc, time::Duration};

// Progressbar animation setup
const PROGRESS_ANIMATION_STEP: f64 = 0.05;
const PROGRESS_ANIMATION_TIME: u64 = 20; //ms

struct Progress {
    fraction: RefCell<f64>,
    source_id: RefCell<Option<SourceId>>,
}

// Main
pub struct Request {
    progress: Arc<Progress>,
    widget: Entry,
}

impl Request {
    // Construct
    pub fn new(
        // Actions
        action_update: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>, // @TODO local `action_page_open`?
    ) -> Self {
        // GTK
        let widget = Entry::builder()
            .placeholder_text("URL or search term...")
            .hexpand(true)
            .build();

        // Connect events
        widget.connect_changed(move |_| {
            action_update.activate(None);
        });

        widget.connect_activate(move |_| {
            action_tab_page_navigation_reload.activate(None);
        });

        // Init animated progressbar state
        let progress = Arc::new(Progress {
            fraction: RefCell::new(0.0),
            source_id: RefCell::new(None),
        });

        // Result
        Self { progress, widget }
    }

    // Actions
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
                            let widget = self.widget.clone();
                            let progress = self.progress.clone();

                            // Frame
                            move || {
                                // Animate
                                if *progress.fraction.borrow() > widget.progress_fraction() {
                                    widget.set_progress_fraction(
                                        // Currently, here is no outrange validation, seems that wrapper make this work @TODO
                                        widget.progress_fraction() + PROGRESS_ANIMATION_STEP,
                                    );
                                    return ControlFlow::Continue;
                                }
                                // Deactivate
                                progress.source_id.replace(None);

                                // Reset (to hide progress widget)
                                widget.set_progress_fraction(0.0);

                                // Stop iteration
                                ControlFlow::Break
                            }
                        },
                    )));
                }
            }
        }
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_page_navigation_id) {
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
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    if let Some(text) = record.text {
                        self.widget.set_text(&text);
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
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        let text = self.widget.text();

        match Database::add(
            transaction,
            app_browser_window_tab_item_page_navigation_id,
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

    // Setters
    pub fn set_text(&self, value: &GString) {
        self.widget.set_text(value);
    }

    // Getters
    pub fn widget(&self) -> &Entry {
        &self.widget
    }

    pub fn is_empty(&self) -> bool {
        0 == self.widget.text_length()
    }

    pub fn text(&self) -> GString {
        self.widget.text()
    }

    pub fn uri(&self) -> Option<Uri> {
        match Uri::parse(&self.widget.text(), UriFlags::NONE) {
            Ok(uri) => Some(uri),
            _ => None,
        }
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        // nothing yet..

        // Success
        Ok(())
    }
}
