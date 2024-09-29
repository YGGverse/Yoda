use gtk::glib::{gformat, markup_escape_text, GString, Regex, RegexCompileFlags, RegexMatchFlags};

pub enum Level {
    H1,
    H2,
    H3,
}

pub struct Header {
    // level: Level,
    text: GString,
    markup: GString,
}

impl Header {
    pub fn from(line: &str) -> Option<Header> {
        // Parse line
        let parsed = Regex::split_simple(
            r"^(#{1,3})\s*(.+)$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Validate match results
        if let Some(text) = parsed.get(2) {
            if let Some(level) = parsed.get(1) {
                // Init level
                let level = match level.len() {
                    1 => Level::H1,
                    2 => Level::H2,
                    3 => Level::H3,
                    _ => return None,
                };

                // Init text
                let text = GString::from(text.as_str());

                if text.trim().is_empty() {
                    return None;
                }

                // Init markup
                let markup = match level {
                    Level::H1 => gformat!(
                        "<span size=\"xx-large\">{}</span>\n",
                        markup_escape_text(&text)
                    ),
                    Level::H2 => gformat!(
                        "<span size=\"x-large\">{}</span>\n",
                        markup_escape_text(&text)
                    ),
                    Level::H3 => gformat!(
                        "<span size=\"large\">{}</span>\n",
                        markup_escape_text(&text)
                    ),
                };

                // Result
                return Some(Header {
                    // level,
                    text,
                    markup,
                });
            }
        }

        None // not header line given
    }

    pub fn text(&self) -> &GString {
        &self.text
    }

    pub fn markup(&self) -> &GString {
        &self.markup
    }
}
