use gtk::glib::{GString, Regex, RegexCompileFlags, RegexMatchFlags, Uri, UriFlags};

pub struct Link {
    pub alt: Option<GString>,
    pub date: Option<GString>, // @TODO https://docs.gtk.org/glib/struct.Date.html
    pub is_external: Option<bool>,
    pub uri: Uri,
}

impl Link {
    pub fn from(line: &str, to_base: Option<&Uri>) -> Option<Link> {
        // Define initial values
        let mut alt = None;
        let mut date = None;
        let mut is_external = None;

        // Begin line parse
        let regex = Regex::split_simple(
            r"^=>\s*([^\s]+)\s*(\d{4}-\d{2}-\d{2})?\s*(.+)?$",
            line,
            RegexCompileFlags::DEFAULT,
            RegexMatchFlags::DEFAULT,
        );

        // Detect address required to continue
        let unresolved_address = regex.get(1)?;

        // Convert address to the valid URI
        let uri = match to_base {
            // Base conversion requested
            Some(base_uri) => {
                // Convert relative address to absolute
                match Uri::resolve_relative(
                    Some(&base_uri.to_str()),
                    unresolved_address.as_str(),
                    UriFlags::NONE,
                ) {
                    Ok(resolved_str) => {
                        // Try convert string to the valid URI
                        match Uri::parse(&resolved_str, UriFlags::NONE) {
                            Ok(resolved_uri) => {
                                // Change external status
                                is_external = Some(
                                    resolved_uri.host() != base_uri.host()
                                        || resolved_uri.port() != base_uri.port(),
                                );

                                // Result
                                resolved_uri
                            }
                            Err(_) => return None,
                        }
                    }
                    Err(_) => return None,
                }
            }
            // Base resolve not requested
            None => {
                // Just try convert address to valid URI
                match Uri::parse(&unresolved_address, UriFlags::NONE) {
                    Ok(unresolved_uri) => unresolved_uri,
                    Err(_) => return None,
                }
            }
        };

        // Date
        if let Some(value) = regex.get(2) {
            date = Some(GString::from(value.as_str()))
        }

        // Alt
        if let Some(value) = regex.get(3) {
            alt = Some(GString::from(value.as_str()))
        };

        Some(Self {
            alt,
            date,
            is_external,
            uri,
        })
    }
}
