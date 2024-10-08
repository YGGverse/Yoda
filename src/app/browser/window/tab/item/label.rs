mod database;
mod pin;
mod title;
mod widget;

use database::Database;
use pin::Pin;
use sqlite::Transaction;
use title::Title;
use widget::Widget;

use gtk::{glib::GString, prelude::WidgetExt, Box, GestureClick};
use std::sync::{Arc, Mutex};

pub struct Label {
    // Components
    pin: Arc<Pin>,
    title: Arc<Title>,
    // GTK
    widget: Arc<Widget>,
    // Extras
    is_pinned: Mutex<bool>,
}

impl Label {
    // Construct
    pub fn new_arc(name: GString, is_pinned: bool) -> Arc<Self> {
        // Init components
        let pin = Pin::new_arc(is_pinned);
        let title = Title::new_arc();
        let widget = Widget::new_arc(name, pin.gobject(), title.gobject());

        // Init label struct
        let label = Arc::new(Self {
            pin: pin.clone(),
            title: title.clone(),
            widget: widget.clone(),
            is_pinned: Mutex::new(is_pinned),
        });

        // Init events:

        // Await for widget realize to continue this feature
        widget.gobject().connect_realize({
            let widget = widget.clone();
            let label = label.clone();
            move |_| {
                // Init GestureClick listener
                let controller = GestureClick::new();

                // Listen for double click
                controller.connect_pressed({
                    let label = label.clone();
                    move |_, count, _, _| {
                        if count == 2 {
                            label.pin(!label.is_pinned());
                        }
                    }
                });

                // Label's parent contain native GTK paddings, that makes click event ignored
                // try assign the controller to parent as the solution
                match widget.gobject().parent() {
                    // @TODO check for GtkGizmo type?
                    Some(parent) => {
                        parent.add_controller(controller);
                    }
                    // Assign controller to the current widget, drop notice
                    None => {
                        widget.gobject().add_controller(controller);
                        println!("Could not assign action to destination, please report");
                        // @TODO
                    }
                }
            }
        });

        // Result
        label
    }

    // Actions
    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
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
        app_browser_window_tab_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    self.pin(record.is_pinned);

                    // Delegate restore action to childs
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
        app_browser_window_tab_id: &i64,
    ) -> Result<(), String> {
        match Database::add(transaction, app_browser_window_tab_id, &self.is_pinned()) {
            Ok(_) => {
                // Delegate save action to childs
                // nothing yet..
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn update(&self, title: Option<&GString>) {
        self.title.update(title);
        self.widget.update(title);
    }

    // Setters
    pub fn pin(&self, is_pinned: bool) {
        // Update Self
        *self.is_pinned.lock().unwrap() = is_pinned;

        // Update child components
        self.pin.pin(is_pinned);
        self.title.pin(is_pinned);
    }

    // Getters
    pub fn is_pinned(&self) -> bool {
        self.is_pinned.lock().unwrap().clone()
    }

    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
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
