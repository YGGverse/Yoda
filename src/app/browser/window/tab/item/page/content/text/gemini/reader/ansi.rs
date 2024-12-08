mod rgba;
mod tag;

use tag::Tag;

use ansi_parser::{AnsiParser, AnsiSequence, Output};
use gtk::{prelude::TextTagExt, TextTag};

/// Apply ANSI/SGR format to new buffer
pub fn format(source_code: &str) -> Vec<(TextTag, String)> {
    let mut buffer = Vec::new();
    let mut tag = Tag::new();

    for ref entity in source_code.ansi_parse() {
        if let Output::Escape(AnsiSequence::SetGraphicsMode(color)) = entity {
            if color.len() > 1 {
                if color[0] == 38 {
                    tag.text_tag
                        .set_foreground_rgba(rgba::default(*color.last().unwrap()).as_ref());
                } else {
                    tag.text_tag
                        .set_background_rgba(rgba::default(*color.last().unwrap()).as_ref());
                }
            }
        }
        if let Output::TextBlock(text) = entity {
            buffer.push((tag.text_tag, text.to_string()));
            tag = Tag::new();
        }
    }

    buffer
}
