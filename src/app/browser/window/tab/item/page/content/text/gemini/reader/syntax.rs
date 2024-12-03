use gtk::{
    gdk::RGBA,
    pango::{Style, Underline},
    prelude::TextTagExt,
    TextTag,
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, FontStyle, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
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

    pub fn highlight(
        &self,
        source_code: &str,
        source_tag: &TextTag,
        alt: Option<&String>,
    ) -> Result<Vec<(TextTag, String)>, Error> {
        if let Some(name) = alt {
            if let Some(reference) = self.syntax_set.find_syntax_by_name(name) {
                return self.syntect_buffer(source_code, source_tag, reference);
            }

            if let Some(reference) = self.syntax_set.find_syntax_by_extension(name) {
                return self.syntect_buffer(source_code, source_tag, reference);
            }
        }

        if let Some(reference) = self.syntax_set.find_syntax_by_first_line(source_code) {
            return self.syntect_buffer(source_code, source_tag, reference);
        }

        Ok(self.default_buffer(source_code, source_tag))
    }

    fn default_buffer(&self, source: &str, source_tag: &TextTag) -> Vec<(TextTag, String)> {
        // Init new line buffer
        let mut buffer = Vec::new();

        // Create new tag preset from source
        let tag = new_text_tag_from(source_tag);

        // Append
        buffer.push((tag, source.to_string()));
        buffer
    }

    fn syntect_buffer(
        &self,
        source: &str,
        source_tag: &TextTag,
        syntax_reference: &SyntaxReference,
    ) -> Result<Vec<(TextTag, String)>, Error> {
        // Init new line buffer
        let mut buffer = Vec::new();

        // Apply syntect decorator
        let ranges = HighlightLines::new(syntax_reference, &self.theme_set.themes[DEFAULT_THEME])
            .highlight_line(&source, &self.syntax_set)?;

        // Build tags
        for (style, entity) in ranges {
            // Create new tag preset from source
            let tag = new_text_tag_from(source_tag);

            // Tuneup using syntect conversion
            // tag.set_background_rgba(Some(&color_to_rgba(style.background)));
            tag.set_foreground_rgba(Some(&color_to_rgba(style.foreground)));
            tag.set_style(font_style_to_style(style.font_style));
            tag.set_underline(font_style_to_underline(style.font_style));

            // Append
            buffer.push((tag, entity.to_string()));
        }

        Ok(buffer)
    }
}

// Tools

fn color_to_rgba(color: Color) -> RGBA {
    /* @TODO #1931
    RGBA::new(
        color.r.into(),
        color.g.into(),
        color.b.into(),
        color.a.into(),
    )*/

    RGBA::parse(format!(
        "rgba({},{},{},{})",
        color.r, color.g, color.b, color.a
    ))
    .unwrap()
}

fn font_style_to_style(font_style: FontStyle) -> Style {
    match font_style {
        FontStyle::ITALIC => Style::Italic,
        _ => Style::Normal,
    }
}

fn font_style_to_underline(font_style: FontStyle) -> Underline {
    match font_style {
        FontStyle::UNDERLINE => Underline::Single,
        _ => Underline::None,
    }
}

fn new_text_tag_from(source_tag: &TextTag) -> TextTag {
    let text_tag = TextTag::builder()
        .left_margin(source_tag.left_margin())
        .scale(source_tag.scale())
        .wrap_mode(source_tag.wrap_mode())
        .build();

    if let Some(ref family) = source_tag.family() {
        text_tag.set_family(Some(family));
    }

    if let Some(ref foreground_rgba) = source_tag.foreground_rgba() {
        text_tag.set_foreground_rgba(Some(foreground_rgba));
    }

    text_tag
}
