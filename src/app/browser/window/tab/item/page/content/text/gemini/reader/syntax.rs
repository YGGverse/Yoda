use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    parsing::{SyntaxReference, SyntaxSet},
    util::as_24_bit_terminal_escaped,
    Error,
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

    pub fn auto_highlight(&self, source: &str, alt: Option<&String>) -> Result<String, Error> {
        if let Some(name) = alt {
            if let Some(syntax_reference) = self.syntax_set.find_syntax_by_name(name) {
                return self.highlight(source, syntax_reference);
            }
        }

        if let Some(syntax_reference) = self.syntax_set.find_syntax_by_first_line(source) {
            return self.highlight(source, syntax_reference);
        }

        Ok(source.to_string())
    }

    fn highlight(&self, source: &str, syntax_reference: &SyntaxReference) -> Result<String, Error> {
        let ranges = HighlightLines::new(syntax_reference, &self.theme_set.themes[DEFAULT_THEME])
            .highlight_line(&source, &self.syntax_set)?;

        Ok(as_24_bit_terminal_escaped(&ranges[..], true))
    }
}
