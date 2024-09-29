use gtk::{
    gio::SimpleAction,
    glib::{timeout_add_local, ControlFlow, GString, Uri, UriFlags},
    prelude::{ActionExt, EditableExt, EntryExt},
    Entry,
};

use std::{sync::Arc, time::Duration};

const PROGRESS_ANIMATION_STEP: f64 = 0.2;
const PROGRESS_ANIMATION_TIME: u64 = 25;

pub struct Request {
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

        // Result
        Self { widget }
    }

    // Actions
    pub fn update(&self, progress_fraction: f64) {
        // Animate progress fraction update
        timeout_add_local(Duration::from_millis(PROGRESS_ANIMATION_TIME), {
            let widget = self.widget.clone();
            move || {
                if progress_fraction > widget.progress_fraction() {
                    widget.set_progress_fraction(
                        widget.progress_fraction() + PROGRESS_ANIMATION_STEP,
                    );
                    return ControlFlow::Continue;
                } else {
                    widget.set_progress_fraction(progress_fraction);
                }
                ControlFlow::Break
            }
        });
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
