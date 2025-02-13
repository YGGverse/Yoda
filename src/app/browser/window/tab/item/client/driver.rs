mod file;
mod gemini;

use super::{Feature, Page};
use file::File;
use gemini::Gemini;
use std::rc::Rc;

/// Different protocols implementation
pub struct Driver {
    pub file: File,
    pub gemini: Gemini,
}

impl Driver {
    // Constructors

    /// Build new `Self`
    pub fn build(page: &Rc<Page>) -> Self {
        Driver {
            file: File::init(page),
            gemini: Gemini::init(page),
        }
    }
}
