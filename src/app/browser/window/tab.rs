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

pub struct Tab {
    // Keep action links in memory to not require them on every tab append
    action_tab_page_navigation_base: Arc<SimpleAction>,
    action_tab_page_navigation_history_back: Arc<SimpleAction>,
    action_tab_page_navigation_history_forward: Arc<SimpleAction>,
    action_tab_page_navigation_reload: Arc<SimpleAction>,
    action_update: Arc<SimpleAction>,
    // Dynamically allocated reference index
    labels: RefCell<HashMap<GString, Arc<Label>>>,
    pages: RefCell<HashMap<GString, Arc<Page>>>,
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
            labels: RefCell::new(HashMap::new()),
            pages: RefCell::new(HashMap::new()),
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
                let id = &widget.widget_name();
                tab.labels.borrow_mut().remove(id);
                tab.pages.borrow_mut().remove(id);
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
        self.labels.borrow_mut().insert(id.clone(), label.clone());
        self.pages.borrow_mut().insert(id.clone(), page.clone());

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
        self.widget
            .gobject()
            .remove_page(self.widget.gobject().current_page());
    }

    // Close all tabs
    pub fn close_all(&self) {
        // @TODO skip pinned or make confirmation alert (GTK>=4.10)
        while let Some(page_number) = self.widget.gobject().current_page() {
            self.widget.gobject().remove_page(Some(page_number));
        }
    }

    // Toggle pin status for active tab
    pub fn pin(&self) {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get label by ID
                if let Some(label) = self.labels.borrow().get(&widget.widget_name()) {
                    label.pin(!label.is_pinned()); // toggle
                }
            }
        }
    }

    pub fn page_navigation_base(&self) {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(&widget.widget_name()) {
                    page.navigation_base();
                }
            }
        }
    }

    pub fn page_navigation_history_back(&self) {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(&widget.widget_name()) {
                    page.navigation_history_back();
                }
            }
        }
    }

    pub fn page_navigation_history_forward(&self) {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(&widget.widget_name()) {
                    page.navigation_history_forward();
                }
            }
        }
    }

    pub fn page_navigation_reload(&self) {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(&widget.widget_name()) {
                    page.navigation_reload();
                }
            }
        }
    }

    pub fn update(&self) {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get widget ID
                let id = &widget.widget_name();

                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(id) {
                    page.update();

                    // Get label by widget ID
                    if let Some(label) = self.labels.borrow().get(id) {
                        if let Some(title) = page.title() {
                            label.update(Some(&title));
                        } else {
                            label.update(None);
                        }
                    }
                }
            }
        }
    }

    // Getters
    pub fn page_title(&self) -> Option<GString> {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get widget ID
                let id = &widget.widget_name();
                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(id) {
                    return page.title();
                }
            }
        }

        None
    }

    pub fn page_description(&self) -> Option<GString> {
        // Get current page
        if let Some(page_number) = self.widget.gobject().current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.gobject().nth_page(Some(page_number)) {
                // Get widget ID
                let id = &widget.widget_name();
                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(id) {
                    return page.description();
                }
            }
        }

        None
    }

    pub fn gobject(&self) -> &Notebook {
        self.widget.gobject()
    }
}
