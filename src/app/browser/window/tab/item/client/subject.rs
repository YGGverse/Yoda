use super::Page;
use adw::TabPage;
use std::rc::Rc;

/// The subject for `Client` handler
pub struct Subject {
    pub page: Rc<Page>,
    pub tab_page: TabPage,
}
