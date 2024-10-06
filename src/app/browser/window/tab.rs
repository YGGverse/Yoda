mod label;
mod page;
mod widget;

use label::Label;
use page::Page;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, WidgetExt},
    GestureClick, Notebook,
};

use std::{cell::RefCell, collections::HashMap, sync::Arc};

// Common struct for HashMap index
struct TabItem {
    label: Arc<Label>,
    page: Arc<Page>,
}

pub struct Tab {
    // Keep action links in memory to not require them on every tab append
    action_tab_page_navigation_base: Arc<SimpleAction>,
    action_tab_page_navigation_history_back: Arc<SimpleAction>,
    action_tab_page_navigation_history_forward: Arc<SimpleAction>,
    action_tab_page_navigation_reload: Arc<SimpleAction>,
    action_update: Arc<SimpleAction>,
    // Dynamically allocated reference index
    index: RefCell<HashMap<GString, TabItem>>,
    // GTK
    widget: Arc<Widget>,
}

impl Tab {
    // Construct
    pub fn new(
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Self {
        // Return non activated struct
        Self {
            // Define action links
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_update,
            // Init empty HashMap index as no tabs appended yet
            index: RefCell::new(HashMap::new()),
            // GTK
            widget: Arc::new(Widget::new()),
        }
    }

    // Actions
    pub fn activate(&self, tab: Arc<Self>) {
        self.widget
            .gobject()
            .connect_page_removed(move |_, widget, _| {
                // Cleanup HashMap index
                let id = widget.widget_name();

                // Check for required value as raw access to gobject @TODO
                if id.is_empty() {
                    panic!("Undefined tab index!")
                }

                tab.index.borrow_mut().remove(&id);
            });

        // Switch page post-event (`connect_switch_page` activates before `page_number` get updated)
        self.widget.gobject().connect_page_notify({
            let action_update = self.action_update.clone();
            // Update window header with current page title
            move |_| action_update.activate(None)
        });
    }

    pub fn append(
        &self,
        page_navigation_request_text: Option<GString>,
        is_current_page: bool,
    ) -> u32 {
        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init new tab components
        let label = Arc::new(Label::new(id.clone(), false));
        let page = Arc::new(Page::new(
            id.clone(),
            page_navigation_request_text.clone(),
            self.action_tab_page_navigation_base.clone(),
            self.action_tab_page_navigation_history_back.clone(),
            self.action_tab_page_navigation_history_forward.clone(),
            self.action_tab_page_navigation_reload.clone(),
            self.action_update.clone(),
        ));

        // Register dynamically created tab components in the HashMap index
        self.index.borrow_mut().insert(
            id.clone(),
            TabItem {
                label: label.clone(),
                page: page.clone(),
            },
        );

        // Init additional label actions
        let controller = GestureClick::new();

        controller.connect_pressed({
            let label = label.clone();
            move |_, count, _, _| {
                // double click
                if count == 2 {
                    label.pin(!label.is_pinned()); // toggle
                }
            }
        });

        label.gobject().add_controller(controller);

        // Append new Notebook page
        let page_number = self
            .widget
            .gobject()
            .append_page(page.widget(), Some(label.gobject()));

        // Additional setup for Notebook tab created
        self.widget
            .gobject()
            .set_tab_reorderable(page.widget(), true);

        if is_current_page {
            self.widget.gobject().set_current_page(Some(page_number));
        }

        if page_navigation_request_text.is_none() {
            page.navigation_request_grab_focus();
        }

        // Result
        page_number
    }

    // Close active tab
    pub fn close(&self) {
        self.widget.close();
    }

    // Close all tabs
    pub fn close_all(&self) {
        self.widget.close_all();
    }

    // Toggle pin status for active tab
    pub fn pin(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.label.pin(!item.label.is_pinned()); // toggle
            }
        }
    }

    pub fn page_navigation_base(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_base();
            }
        }
    }

    pub fn page_navigation_history_back(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_history_back();
            }
        }
    }

    pub fn page_navigation_history_forward(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_history_forward();
            }
        }
    }

    pub fn page_navigation_reload(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_reload();
            }
        }
    }

    pub fn update(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.update();
                if let Some(title) = item.page.title() {
                    item.label.update(Some(&title));
                } else {
                    item.label.update(None);
                }
            }
        }
    }

    // Getters
    pub fn page_title(&self) -> Option<GString> {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                return item.page.title();
            }
        }

        None
    }

    pub fn page_description(&self) -> Option<GString> {
        if let Some(id) = self.widget.current_name() {
            // Get page by widget ID
            if let Some(item) = self.index.borrow().get(&id) {
                return item.page.description();
            }
        }

        None
    }

    pub fn gobject(&self) -> &Notebook {
        self.widget.gobject()
    }
}
