use gtk::glib::{Uri, UriFlags};

pub struct Reference {
    pub uri: Uri,
    pub alt: String,
}

impl Reference {
    pub fn parse(address: &str, alt: Option<&str>, base: &Uri) -> Option<Self> {
        // Convert address to the valid URI,
        // resolve to absolute URL format if the target is relative
        match Uri::resolve_relative(
            Some(&base.to_string()),
            // Relative scheme patch
            // https://datatracker.ietf.org/doc/html/rfc3986#section-4.2
            &match address.strip_prefix("//") {
                Some(p) => {
                    let s = p.trim_start_matches(":");
                    format!(
                        "{}://{}",
                        base.scheme(),
                        if s.is_empty() {
                            format!("{}/", base.host().unwrap_or_default())
                        } else {
                            s.into()
                        }
                    )
                }
                None => String::new(),
            },
            UriFlags::NONE,
        ) {
            Ok(ref url) => match Uri::parse(url, UriFlags::NONE) {
                Ok(uri) => {
                    let mut a: Vec<&str> = Vec::with_capacity(2);
                    if uri.scheme() != base.scheme() {
                        a.push("⇖");
                    }
                    match alt {
                        Some(text) => a.push(text),
                        None => a.push(url),
                    }
                    Some(Self {
                        uri,
                        alt: a.join(" "),
                    })
                }
                Err(_) => todo!(),
            },
            Err(_) => None,
        }
    }
}
