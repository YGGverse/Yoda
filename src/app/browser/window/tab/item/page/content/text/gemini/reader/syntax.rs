pub mod error;
pub use error::Error;

use adw::StyleManager;
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
};

/*  Default theme
    @TODO make optional
    base16-ocean.dark
    base16-eighties.dark
    base16-mocha.dark
    base16-ocean.light
    InspiredGitHub
    Solarized (dark)
    Solarized (light)
*/
pub const DEFAULT_THEME_DARK: &str = "base16-eighties.dark";
pub const DEFAULT_THEME_LIGHT: &str = "InspiredGitHub";

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
        if let Some(value) = alt {
            if let Some(reference) = self.syntax_set.find_syntax_by_name(value) {
                return self.buffer(source_code, source_tag, reference);
            }

            if let Some(reference) = self.syntax_set.find_syntax_by_token(value) {
                return self.buffer(source_code, source_tag, reference);
            }

            if let Some(reference) = self.syntax_set.find_syntax_by_path(value) {
                return self.buffer(source_code, source_tag, reference);
            }
        }

        if let Some(reference) = self.syntax_set.find_syntax_by_first_line(source_code) {
            return self.buffer(source_code, source_tag, reference);
        }

        Err(Error::Parse)
    }

    fn buffer(
        &self,
        source: &str,
        source_tag: &TextTag,
        syntax_reference: &SyntaxReference,
    ) -> Result<Vec<(TextTag, String)>, Error> {
        // Init new line buffer
        let mut buffer = Vec::new();

        // Apply syntect decorator
        let mut ranges = HighlightLines::new(
            syntax_reference,
            &self.theme_set.themes[if StyleManager::default().is_dark() {
                DEFAULT_THEME_DARK
            } else {
                DEFAULT_THEME_LIGHT
            }], // @TODO apply on env change
        );

        match ranges.highlight_line(source, &self.syntax_set) {
            Ok(result) => {
                // Build tags
                for (style, entity) in result {
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
            Err(e) => Err(Error::Syntect(e)),
        }
    }
}

// Tools

fn color_to_rgba(color: Color) -> RGBA {
    RGBA::new(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        color.a as f32 / 255.0,
    )
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
