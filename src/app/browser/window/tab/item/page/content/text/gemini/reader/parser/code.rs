pub mod inline;
pub mod multiline;

use inline::Inline;
use multiline::Multiline;

pub struct Code {
    // nothing yet..
}

impl Code {
    // Inline
    pub fn inline_from(line: &str) -> Option<Inline> {
        Inline::from(line)
    }

    // Multiline
    pub fn multiline_begin_from(line: &str) -> Option<Multiline> {
        Multiline::begin_from(line)
    }

    pub fn multiline_continue_from(this: &mut Multiline, line: &str) {
        Multiline::continue_from(this, line)
    }
}
