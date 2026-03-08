use gtk::glib::{Uri, UriFlags};

pub const REGEX_LINK: &str = r"\[(?P<text>[^\]]+)\]\((?P<url>[^\)]+)\)";

pub const REGEX_IMAGE_LINK: &str =
    r"\[(?P<is_img>!)\[(?P<alt>[^\]]+)\]\((?P<img_url>[^\)]+)\)\]\((?P<link_url>[^\)]+)\)";

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
        position: &mut gtk::TextIter,
        link_color: &gtk::gdk::RGBA,
        tag: &super::Tag,
        is_annotation: bool,
        links: &mut std::collections::HashMap<gtk::TextTag, Uri>,
    ) {
        use gtk::{TextTag, WrapMode, prelude::TextBufferExtManual};
        let a = if is_annotation {
            buffer.insert_with_tags(position, " ", &[]);
            TextTag::builder()
                .foreground_rgba(link_color)
                // .foreground_rgba(&adw::StyleManager::default().accent_color_rgba())
                // @TODO adw 1.6 / ubuntu 24.10+
                .pixels_above_lines(4)
                .pixels_below_lines(4)
                .rise(5000)
                .scale(0.8)
                .wrap_mode(WrapMode::Word)
                .build()
        } else {
            TextTag::builder()
                .foreground_rgba(link_color)
                // .foreground_rgba(&adw::StyleManager::default().accent_color_rgba())
                // @TODO adw 1.6 / ubuntu 24.10+
                .sentence(true)
                .wrap_mode(WrapMode::Word)
                .build()
        };
        if !tag.text_tag_table.add(&a) {
            panic!()
        }
        buffer.insert_with_tags(position, &self.alt, &[&a]);
        links.insert(a, self.uri);
    }
}

#[test]
fn test_regex_link() {
    let cap: Vec<_> = regex::Regex::new(REGEX_LINK)
        .unwrap()
        .captures_iter(r#"[link1](https://link1.com) [link2](https://link2.com)"#)
        .collect();

    let first = cap.get(0).unwrap();
    assert_eq!(&first["text"], "link1");
    assert_eq!(&first["url"], "https://link1.com");

    let second = cap.get(1).unwrap();
    assert_eq!(&second["text"], "link2");
    assert_eq!(&second["url"], "https://link2.com");
}

#[test]
fn test_regex_image_link() {
    let cap: Vec<_> = regex::Regex::new(
        REGEX_IMAGE_LINK,
    )
    .unwrap().captures_iter(
        r#"[![image1](https://image1.com)](https://image2.com) [![image3](https://image3.com)](https://image4.com)"#
    ).collect();

    let first = cap.get(0).unwrap();
    assert_eq!(&first["alt"], "image1");
    assert_eq!(&first["img_url"], "https://image1.com");
    assert_eq!(&first["link_url"], "https://image2.com");

    let second = cap.get(1).unwrap();
    assert_eq!(&second["alt"], "image3");
    assert_eq!(&second["img_url"], "https://image3.com");
    assert_eq!(&second["link_url"], "https://image4.com");
}
