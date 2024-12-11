mod database;

use crate::app::browser::{window::tab::item::Action as TabAction, Action as BrowserAction};
use gtk::{
    glib::{timeout_add_local, ControlFlow, SourceId},
    prelude::{EditableExt, EntryExt, WidgetExt},
    Entry, EntryIconPosition, StateFlags,
};
use sqlite::Transaction;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};

const PLACEHOLDER_TEXT: &str = "URL or search term...";

const GO_TOOLTIP_TEXT: &str = "Go to the address";
const GO_ICON_NAME: &str = "pan-end-symbolic";

const IDENTITY_TOOLTIP_TEXT: (&str, &str) = (
    "Identity",
    "Identity feature not available for this location",
);
const IDENTITY_ICON_NAME: (&str, &str) =
    ("avatar-default-symbolic", "applications-system-symbolic");

// Progress bar animation setup
const PROGRESS_ANIMATION_STEP: f64 = 0.05;
const PROGRESS_ANIMATION_TIME: u64 = 20; //ms

struct Progress {
    fraction: RefCell<f64>,
    source_id: RefCell<Option<SourceId>>,
}

pub struct Widget {
    pub entry: Entry,
    progress: Rc<Progress>,
}

impl Widget {
    // Construct
    pub fn new(action: (Rc<BrowserAction>, Rc<TabAction>)) -> Self {
        // Set actions name
        let (browser_action, tab_action) = action;

        // Init animated progress bar state
        let progress = Rc::new(Progress {
            fraction: RefCell::new(0.0),
            source_id: RefCell::new(None),
        });

        // Init main widget
        let entry = Entry::builder()
            .placeholder_text(PLACEHOLDER_TEXT)
            .hexpand(true)
            .build();

        entry.set_primary_icon_name(Some(IDENTITY_ICON_NAME.0));
        entry.set_primary_icon_tooltip_text(Some(IDENTITY_TOOLTIP_TEXT.0));

        entry.set_secondary_icon_name(Some(GO_ICON_NAME));
        entry.set_secondary_icon_tooltip_text(Some(GO_TOOLTIP_TEXT));

        // Connect events
        entry.connect_icon_release({
            let tab_action = tab_action.clone();
            move |this, position| match position {
                EntryIconPosition::Primary => tab_action.ident.activate(),
                EntryIconPosition::Secondary => tab_action.load.activate(Some(&this.text()), true),
                _ => todo!(), // unexpected
            }
        });

        entry.connect_changed(move |_| {
            browser_action.update.activate(None);
        });

        entry.connect_activate(move |entry| {
            tab_action.load.activate(Some(&entry.text()), true);
        });

        entry.connect_state_flags_changed({
            // Define last focus state container
            let has_focus = Cell::new(false);
            move |entry, state| {
                // Select entire text on first click (release)
                // this behavior implemented in most web-browsers,
                // to simply overwrite current request with new value
                // Note:
                // * Custom GestureClick is not an option here, as GTK Entry has default controller
                // * This is experimental feature does not follow native GTK behavior @TODO make optional
                if !has_focus.take()
                    && state.contains(StateFlags::ACTIVE | StateFlags::FOCUS_WITHIN)
                    && entry.selection_bounds().is_none()
                {
                    entry.select_region(0, entry.text_length().into());
                }
                // Update last focus state
                has_focus.replace(state.contains(StateFlags::FOCUS_WITHIN));
            }
        });

        // Return activated `Self`
        Self { entry, progress }
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
                        self.entry.set_text(&text);
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
        let text = self.entry.text();

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

    pub fn update(
        &self,
        progress_fraction: Option<f64>,
        is_identity_applicable: bool,
        is_identity_active: bool,
    ) {
        // Update identity
        self.entry
            .set_primary_icon_activatable(is_identity_applicable);
        self.entry
            .set_primary_icon_sensitive(is_identity_applicable);

        if is_identity_applicable {
            self.entry.set_primary_icon_name(Some(IDENTITY_ICON_NAME.0));
            self.entry
                .set_primary_icon_tooltip_text(Some(IDENTITY_TOOLTIP_TEXT.0));
        } else {
            self.entry.set_primary_icon_name(Some(IDENTITY_ICON_NAME.1));
            self.entry
                .set_primary_icon_tooltip_text(Some(IDENTITY_TOOLTIP_TEXT.1));
        }

        let identity = self.entry.first_child().unwrap(); // @TODO handle
        identity.remove_css_class("success");
        if is_identity_active {
            identity.add_css_class("success");
        }

        // Update progress
        // * skip update animation for None value
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
                            let entry = self.entry.clone();
                            let progress = self.progress.clone();

                            // Frame
                            move || {
                                // Animate
                                if *progress.fraction.borrow() > entry.progress_fraction() {
                                    entry.set_progress_fraction(
                                        // Currently, here is no outrange validation, seems that wrapper make this work @TODO
                                        entry.progress_fraction() + PROGRESS_ANIMATION_STEP,
                                    );
                                    return ControlFlow::Continue;
                                }
                                // Deactivate
                                progress.source_id.replace(None);

                                // Reset on 100% (to hide progress bar)
                                // or, just await for new value request
                                if entry.progress_fraction() == 1.0 {
                                    entry.set_progress_fraction(0.0);
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
