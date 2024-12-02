use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet,
    util::as_24_bit_terminal_escaped,
};

pub const DEFAULT_THEME: &str = "base16-ocean.dark";

pub struct Syntax {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight(&self, source: &str, extension: Option<&String>) -> Option<String> {
        match extension {
            Some(extension) => match self.syntax_set.find_syntax_by_extension(extension) {
                Some(syntax) => {
                    let ranges = HighlightLines::new(syntax, &self.theme_set.themes[DEFAULT_THEME])
                        .highlight_line(&source, &self.syntax_set)
                        .unwrap(); // @TODO
                    Some(as_24_bit_terminal_escaped(&ranges[..], true))
                }
                None => None,
            },
            None => None, // @TODO detect by source
        }
    }
}
