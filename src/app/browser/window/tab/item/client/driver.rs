mod gemini;

use super::{Feature, Page};
use adw::TabPage;
use gemini::Gemini;
use std::rc::Rc;

/// Different protocols implementation
pub struct Driver {
    pub gemini: Gemini,
}

impl Driver {
    // Constructors

    /// Build new `Self`
    pub fn build(page: &Rc<Page>, tab_page: &TabPage) -> Self {
        Driver {
            gemini: Gemini::init(page, tab_page),
        }
    }
}
