mod tag;
use tag::Tag;

use cansi::{v3::categorise_text, Color};
use gtk::{
    gdk::RGBA,
    pango::{Style, Underline},
    prelude::TextTagExt,
    TextTag,
};

/// Apply `ANSI`/`SGR` format to new buffer
pub fn format(source_code: &str) -> Vec<(TextTag, String)> {
    // Init new line buffer
    let mut buffer = Vec::new();

    // Begin entities parse
    for entity in categorise_text(source_code) {
        // Create new tag from default preset
        let tag = Tag::new();

        // Apply supported decorations
        if let Some(color) = entity.fg {
            tag.text_tag
                .set_foreground_rgba(Some(&color_to_rgba(color)));
        }

        if let Some(color) = entity.bg {
            tag.text_tag
                .set_background_rgba(Some(&color_to_rgba(color)));
        }

        if let Some(is_italic) = entity.italic {
            if is_italic {
                tag.text_tag.set_style(Style::Italic);
            }
        }

        if let Some(is_underline) = entity.underline {
            if is_underline {
                tag.text_tag.set_underline(Underline::Single);
            }
        }

        if let Some(is_strikethrough) = entity.strikethrough {
            tag.text_tag.set_strikethrough(is_strikethrough);
        }

        // Append
        buffer.push((tag.text_tag, entity.text.to_string()));
    }

    buffer
}

fn color_to_rgba(value: Color) -> RGBA {
    match value {
        Color::Black => RGBA::new(0.0, 0.0, 0.0, 1.0),
        Color::Red => RGBA::new(0.8, 0.0, 0.0, 1.0),
        Color::Green => RGBA::new(0.0, 0.8, 0.0, 1.0),
        Color::Yellow => RGBA::new(0.8, 0.8, 0.0, 1.0),
        Color::Blue => RGBA::new(0.0, 0.0, 0.9, 1.0),
        Color::Magenta => RGBA::new(0.8, 0.0, 0.8, 1.0),
        Color::Cyan => RGBA::new(0.0, 0.8, 0.8, 1.0),
        Color::White => RGBA::new(0.9, 0.9, 0.9, 1.0),
        Color::BrightBlack => RGBA::new(0.5, 0.5, 0.5, 1.0),
        Color::BrightRed => RGBA::new(1.0, 0.0, 0.0, 1.0),
        Color::BrightGreen => RGBA::new(0.0, 1.0, 0.0, 1.0),
        Color::BrightYellow => RGBA::new(1.0, 1.0, 0.0, 1.0),
        Color::BrightBlue => RGBA::new(0.4, 0.4, 1.0, 1.0),
        Color::BrightMagenta => RGBA::new(1.0, 0.0, 1.0, 1.0),
        Color::BrightCyan => RGBA::new(0.0, 1.0, 1.0, 1.0),
        Color::BrightWhite => RGBA::new(1.0, 1.0, 1.0, 1.0),
    }
}
