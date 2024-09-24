mod label;
mod page;

use label::Label;
use page::Page;

use gtk::{
    glib::{uuid_string_random, GString},
    prelude::WidgetExt,
    GestureClick, Notebook, Widget,
};

use std::{cell::RefCell, collections::HashMap, sync::Arc};

pub struct Tab {
    // GTK
    widget: Arc<Notebook>,
    // Dynamically allocated reference index
    labels: RefCell<HashMap<GString, Arc<Label>>>,
    pages: RefCell<HashMap<GString, Arc<Page>>>,
}

impl Tab {
    // Construct
    pub fn new() -> Arc<Tab> {
        // Init GTK component
        let notebook = Arc::new(Notebook::builder().scrollable(true).build());

        // Init new Tab struct
        let tab = Arc::new(Self {
            // Reference wanted for async events, create new smart pointer
            widget: notebook.clone(),
            // Init empty HashMap index as no tabs appended yet
            labels: RefCell::new(HashMap::new()),
            pages: RefCell::new(HashMap::new()),
        });

        // Connect events
        notebook.connect_page_removed({
            // Make new local ref
            let tab = tab.clone();
            // Begin async action
            move |_, widget: &Widget, _| {
                // Cleanup HashMap index
                let id = &widget.widget_name();
                tab.labels.borrow_mut().remove(id);
                tab.pages.borrow_mut().remove(id);
            }
        });

        tab // return Arc pointer to the new Tab constructed
    }

    // Actions
    pub fn append(&self, is_current_page: bool) -> u32 {
        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init new tab components
        let label = Arc::new(Label::new(id.clone(), false));
        let page = Arc::new(Page::new(id.clone()));

        // Register dynamically created tab components in the HashMap index
        self.labels.borrow_mut().insert(id.clone(), label.clone());
        self.pages.borrow_mut().insert(id.clone(), page.clone());

        // Init additional label actions
        let controller = GestureClick::new();

        controller.connect_pressed({
            let label = label.clone();
            move |_, n: i32, _, _| {
                // double click
                if n == 2 {
                    label.pin(!label.is_pinned()); // toggle
                }
            }
        });

        label.widget().add_controller(controller);

        // Append new Notebook page
        let page_number = self.widget.append_page(page.widget(), Some(label.widget()));

        // Additional setup for Notebook tab created
        self.widget.set_tab_reorderable(page.widget(), true);

        if is_current_page {
            self.widget.set_current_page(Some(page_number));
        }

        // Result
        page_number
    }

    // Close active tab
    pub fn close(&self) {
        self.widget.remove_page(self.widget.current_page());
    }

    // Close all tabs
    pub fn close_all(&self) {
        // @TODO skip pinned or make confirmation alert (GTK>=4.10)
        while let Some(page_number) = self.widget.current_page() {
            self.widget.remove_page(Some(page_number));
        }
    }

    // Toggle pin status for active tab
    pub fn pin(&self) {
        // Get current page
        if let Some(page_number) = self.widget.current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.nth_page(Some(page_number)) {
                // Get label by ID
                if let Some(label) = self.labels.borrow().get(&widget.widget_name()) {
                    label.pin(!label.is_pinned()); // toggle
                }
            }
        }
    }

    pub fn update(&self) {
        // Get current page
        if let Some(page_number) = self.widget.current_page() {
            // Get default widget to extract it name as the ID for childs
            if let Some(widget) = self.widget.nth_page(Some(page_number)) {
                // Get widget ID
                let id = &widget.widget_name();

                // Get label by widget ID
                if let Some(label) = self.labels.borrow().get(id) {
                    label.update();
                }

                // Get page by widget ID
                if let Some(page) = self.pages.borrow().get(id) {
                    page.update();
                }
            }
        }
    }

    // Getters
    pub fn widget(&self) -> &Notebook {
        self.widget.as_ref()
    }
}
