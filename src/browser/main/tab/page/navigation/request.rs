use gtk::{
    gio::SimpleAction,
    glib::{timeout_add_local, ControlFlow, GString, SourceId, Uri, UriFlags},
    prelude::{ActionExt, EditableExt, EntryExt},
    Entry,
};

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
        text: Option<GString>,
        // Actions
        action_update: Arc<SimpleAction>,
        action_tab_page_reload: Arc<SimpleAction>,
    ) -> Self {
        // GTK
        let widget = Entry::builder()
            .placeholder_text("URL or search term...")
            .hexpand(true)
            .text(match text {
                Some(text) => text,
                None => GString::new(),
            })
            .build();

        // Connect events
        widget.connect_changed(move |_| {
            action_update.activate(None);
        });

        widget.connect_activate(move |_| {
            action_tab_page_reload.activate(None);
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

    // Setters
    pub fn set_text(&self, value: &GString, activate: bool) {
        self.widget.set_text(value);

        if activate {
            self.widget.emit_activate();
        }
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
}
