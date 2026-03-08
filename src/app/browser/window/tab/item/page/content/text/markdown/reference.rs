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
                None => address.into(),
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
    pub fn into_buffer(
        self,
        buffer: &gtk::TextBuffer,
        link_color: &gtk::gdk::RGBA,
        tag: &super::Tag,
        links: &mut std::collections::HashMap<gtk::TextTag, Uri>,
    ) {
        use gtk::prelude::{TextBufferExt, TextBufferExtManual};
        let a = gtk::TextTag::builder()
            .foreground_rgba(link_color)
            // .foreground_rgba(&adw::StyleManager::default().accent_color_rgba())
            // @TODO adw 1.6 / ubuntu 24.10+
            .sentence(true)
            .wrap_mode(gtk::WrapMode::Word)
            .build();
        if !tag.text_tag_table.add(&a) {
            panic!()
        }
        buffer.insert_with_tags(&mut buffer.end_iter(), &self.alt, &[&a]);
        links.insert(a, self.uri);
    }
}
