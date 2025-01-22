mod gemini;

use super::{Feature, Subject};
use gemini::Gemini;
use std::rc::Rc;

/// Different protocols implementation
pub struct Driver {
    pub gemini: Gemini,
}

impl Driver {
    // Constructors

    /// Build new `Self`
    pub fn build(subject: &Rc<Subject>) -> Self {
        Driver {
            gemini: Gemini::init(subject),
        }
    }
}
