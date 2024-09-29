use gtk::glib::{
    gformat, markup_escape_text, GString, Regex, RegexCompileFlags, RegexMatchFlags, Uri, UriFlags,
};

pub struct Link {
    //  alt: Option<GString>,  // [optional] alternative text
    //  date: Option<GString>, // [optional] date @TODO store in UnixTime?
    //  external: bool,        // external link indicator
    //  link: GString,         // original link, wanted for title tooltip
    markup: GString, // pango markup with escaped special chars
                     //  uri: Uri,              // parsed link object (currently not in use)
}

impl Link {
    // Link structure parser
    // line - gemtext subject to parse
    // base - Uri object, required for:
    //    1. relative to absolute address conversion
    //    2. external links indication
    // returns new Link struct or None
    pub fn from(line: &str, base: &Uri) -> Option<Link> {
        // Init struct members
        // let mut alt: Option<GString> = None;
        // let mut date: Option<GString> = None;
        let external: bool;
        let link: GString;
        let markup: GString;
        let uri: Uri;

        // Parse line
        let parsed = Regex::split_simple(
            r"^=>\s*([^\s]+)\s*(\d{4}-\d{2}-\d{2})?\s*(.+)?$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Address
        match parsed.get(1) {
            Some(address) => {
                // Define original link value (used in titles or when alt is empty)
                link = GString::from(address.as_str());
                // Links in document usually relative, make them absolute to base given
                match Uri::resolve_relative(Some(&base.to_str()), address.as_str(), UriFlags::NONE)
                {
                    Ok(resolved) => {
                        // Make URI parsed as always valid (no idea why does lib operate strings, not objects)
                        match Uri::parse(&resolved, UriFlags::NONE) {
                            Ok(object) => {
                                // Set external status
                                external =
                                    object.host() != base.host() || object.port() != base.port();

                                // Set struct URI
                                uri = object;
                            }
                            Err(_) => return None,
                        }
                    }
                    Err(_) => return None,
                }
            }
            None => return None,
        }

        // Create link name based on external status, date and alt values
        let mut name = Vec::new();

        if external {
            name.push("⇖".to_string());
        }

        // Date
        if let Some(this) = parsed.get(2) {
            // date = Some(GString::from(this.to_string()));
            name.push(this.to_string());
        }

        // Alt
        match parsed.get(3) {
            // Not empty
            Some(this) => {
                // alt = Some(GString::from(this.to_string()));
                name.push(this.to_string());
            }
            // Empty, use resolved address
            None => name.push(link.to_string()),
        };

        // Markup
        markup = gformat!(
            "<a href=\"{}\" title=\"{}\"><span underline=\"none\">{}</span></a>\n",
            markup_escape_text(&uri.to_str()), // use resolved address for href
            markup_escape_text(&link),         // show original address for title
            markup_escape_text(&name.join(" ")),
        );

        Some(Self {
            // alt,
            // date,
            // external,
            // link,
            markup,
            // uri,
        })
    }

    // Getters
    /* @TODO
    pub fn alt(&self) -> &Option<GString> {
        &self.alt
    }

    pub fn date(&self) -> &Option<GString> {
        &self.date
    }

    pub fn external(&self) -> &bool {
        &self.external
    }

    pub fn link(&self) -> &GString {
        &self.link
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }*/

    pub fn markup(&self) -> &GString {
        &self.markup
    }
}
