mod database;

use crate::app::browser::{window::tab::item::Action as TabAction, Action as BrowserAction};
use gtk::{
    glib::{timeout_add_local, ControlFlow, SourceId},
    prelude::{EditableExt, EntryExt, WidgetExt},
    Entry, StateFlags,
};
use sqlite::Transaction;
use std::{cell::RefCell, rc::Rc, time::Duration};

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
    progress: Rc<Progress>,
}

impl Widget {
    // Construct
    pub fn new(action: (Rc<BrowserAction>, Rc<TabAction>)) -> Self {
        // Init animated progress bar state
        let progress = Rc::new(Progress {
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
            action.0.update().activate(None);
        });

        gobject.connect_activate(move |this| {
            action.1.load().activate(Some(&this.text()), true);
        });

        gobject.connect_state_flags_changed({
            // Define last focus state container
            let has_focus = RefCell::new(false);
            move |this, state| {
                // Select entire text on first click release
                // this behavior implemented in most web-browsers,
                // to simply overwrite current request with new value
                // Note:
                // * Custom GestureClick is not an option here, as GTK Entry has default controller
                // * This is experimental feature does not follow native GTK behavior @TODO make optional
                if !has_focus.take()
                    && state.contains(StateFlags::ACTIVE | StateFlags::FOCUS_WITHIN)
                    && this.selection_bounds().is_none()
                {
                    this.select_region(0, this.text_length().into());
                }
                // Update last focus state
                has_focus.replace(state.contains(StateFlags::FOCUS_WITHIN));
            }
        });

        // Return activated `Self`
        Self { gobject, progress }
    }

    // Actions
    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_request_id: &i64,
    ) -> Result<(), String> {
        match database::select(
            transaction,
            app_browser_window_tab_item_page_navigation_request_id,
        ) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
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
        match database::select(
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

        match database::insert(
            transaction,
            app_browser_window_tab_item_page_navigation_request_id,
            match text.is_empty() {
                true => None,
                false => Some(text.as_str()),
            },
        ) {
            Ok(_) => {
                // let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                // nothing yet..
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn update(&self, progress_fraction: Option<f64>) {
        // Skip update animation for None value
        if let Some(value) = progress_fraction {
            // Update shared fraction on new value was changed
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

                                // Reset on 100% (to hide progress bar)
                                // or, just await for new value request
                                if gobject.progress_fraction() == 1.0 {
                                    gobject.set_progress_fraction(0.0);
                                }

                                // Stop iteration
                                ControlFlow::Break
                            }
                        },
                    )));
                }
            }
        }
    }

    // Getters

    pub fn gobject(&self) -> &Entry {
        &self.gobject
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}
