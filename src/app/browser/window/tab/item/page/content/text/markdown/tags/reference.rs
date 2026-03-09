use gtk::{
    TextBuffer, TextIter, TextTag, WrapMode,
    gdk::RGBA,
    glib::{Uri, UriFlags},
    prelude::{TextBufferExt, TextBufferExtManual},
};
use regex::Regex;
use std::collections::HashMap;

const REGEX_LINK: &str = r"\[(?P<text>[^\]]*)\]\((?P<url>[^\)]+)\)";
const REGEX_IMAGE: &str = r"!\[(?P<alt>[^\]]*)\]\((?P<url>[^\)]+)\)";
const REGEX_IMAGE_LINK: &str =
    r"\[(?P<is_img>!)\[(?P<alt>[^\]]*)\]\((?P<img_url>[^\)]+)\)\]\((?P<link_url>[^\)]+)\)";

struct Reference {
    uri: Uri,
    alt: String,
}

impl Reference {
    /// Try construct new `Self` with given options
    fn parse(address: &str, alt: Option<&str>, base: &Uri) -> Option<Self> {
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

    /// Insert `Self` into the given `TextBuffer` by registering new `TextTag` created
    fn into_buffer(
        self,
        buffer: &TextBuffer,
        position: &mut TextIter,
        link_color: &RGBA,
        is_annotation: bool,
        links: &mut HashMap<TextTag, Uri>,
    ) {
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
        assert!(buffer.tag_table().add(&a));
        buffer.insert_with_tags(position, &self.alt, &[&a]);
        links.insert(a, self.uri);
    }
}

/// Image links `[![]()]()`
pub fn render_images_links(
    buffer: &TextBuffer,
    base: &Uri,
    link_color: &RGBA,
    links: &mut HashMap<TextTag, Uri>,
) {
    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(REGEX_IMAGE_LINK)
        .unwrap()
        .captures_iter(&full_content)
        .collect();

    for cap in matches.into_iter().rev() {
        let full_match = cap.get(0).unwrap();

        let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
        let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

        let mut start_iter = buffer.iter_at_offset(start_char_offset);
        let mut end_iter = buffer.iter_at_offset(end_char_offset);

        buffer.delete(&mut start_iter, &mut end_iter);

        if let Some(this) = Reference::parse(
            &cap["img_url"],
            if cap["alt"].is_empty() {
                None
            } else {
                Some(&cap["alt"])
            },
            base,
        ) {
            this.into_buffer(buffer, &mut start_iter, link_color, false, links)
        }
        if let Some(this) = Reference::parse(&cap["link_url"], Some("1"), base) {
            this.into_buffer(buffer, &mut start_iter, link_color, true, links)
        }
    }
}
/// Image tags `![]()`
pub fn render_images(
    buffer: &TextBuffer,
    base: &Uri,
    link_color: &RGBA,
    links: &mut HashMap<TextTag, Uri>,
) {
    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(REGEX_IMAGE)
        .unwrap()
        .captures_iter(&full_content)
        .collect();

    for cap in matches.into_iter().rev() {
        let full_match = cap.get(0).unwrap();

        let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
        let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

        let mut start_iter = buffer.iter_at_offset(start_char_offset);
        let mut end_iter = buffer.iter_at_offset(end_char_offset);

        buffer.delete(&mut start_iter, &mut end_iter);

        if let Some(this) = Reference::parse(
            &cap["url"],
            if cap["alt"].is_empty() {
                None
            } else {
                Some(&cap["alt"])
            },
            base,
        ) {
            this.into_buffer(buffer, &mut start_iter, link_color, false, links)
        }
    }
}
/// Links `[]()`
pub fn render_links(
    buffer: &TextBuffer,
    base: &Uri,
    link_color: &RGBA,
    links: &mut HashMap<TextTag, Uri>,
) {
    let (start, end) = buffer.bounds();
    let full_content = buffer.text(&start, &end, true).to_string();

    let matches: Vec<_> = Regex::new(REGEX_LINK)
        .unwrap()
        .captures_iter(&full_content)
        .collect();

    for cap in matches.into_iter().rev() {
        let full_match = cap.get(0).unwrap();

        let start_char_offset = full_content[..full_match.start()].chars().count() as i32;
        let end_char_offset = full_content[..full_match.end()].chars().count() as i32;

        let mut start_iter = buffer.iter_at_offset(start_char_offset);
        let mut end_iter = buffer.iter_at_offset(end_char_offset);

        buffer.delete(&mut start_iter, &mut end_iter);

        if let Some(this) = Reference::parse(
            &cap["url"],
            if cap["text"].is_empty() {
                None
            } else {
                Some(&cap["text"])
            },
            base,
        ) {
            this.into_buffer(buffer, &mut start_iter, link_color, false, links)
        }
    }
}

pub fn strip_tags(value: &str) -> String {
    let mut result = String::from(value);
    for cap in Regex::new(REGEX_LINK).unwrap().captures_iter(value) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    result
}

#[test]
fn test_strip_tags() {
    const VALUE: &str = r"Some text [link1](https://link1.com) [link2](https://link2.com)";
    let mut result = String::from(VALUE);
    for cap in Regex::new(REGEX_LINK).unwrap().captures_iter(VALUE) {
        if let Some(m) = cap.get(0) {
            result = result.replace(m.as_str(), &cap["text"]);
        }
    }
    assert_eq!(result, "Some text link1 link2")
}

#[test]
fn test_regex_link() {
    let cap: Vec<_> = Regex::new(REGEX_LINK)
        .unwrap()
        .captures_iter(r"[link1](https://link1.com) [link2](https://link2.com)")
        .collect();

    let first = cap.first().unwrap();
    assert_eq!(&first[0], "[link1](https://link1.com)");
    assert_eq!(&first["text"], "link1");
    assert_eq!(&first["url"], "https://link1.com");

    let second = cap.get(1).unwrap();
    assert_eq!(&second[0], "[link2](https://link2.com)");
    assert_eq!(&second["text"], "link2");
    assert_eq!(&second["url"], "https://link2.com");
}

#[test]
fn test_regex_image_link() {
    let cap: Vec<_> = Regex::new(
        REGEX_IMAGE_LINK,
    )
    .unwrap().captures_iter(
        r"[![image1](https://image1.com)](https://image2.com) [![image3](https://image3.com)](https://image4.com)"
    ).collect();

    let first = cap.first().unwrap();
    assert_eq!(
        &first[0],
        "[![image1](https://image1.com)](https://image2.com)"
    );
    assert_eq!(&first["alt"], "image1");
    assert_eq!(&first["img_url"], "https://image1.com");
    assert_eq!(&first["link_url"], "https://image2.com");

    let second = cap.get(1).unwrap();
    assert_eq!(
        &second[0],
        "[![image3](https://image3.com)](https://image4.com)"
    );
    assert_eq!(&second["alt"], "image3");
    assert_eq!(&second["img_url"], "https://image3.com");
    assert_eq!(&second["link_url"], "https://image4.com");
}

#[test]
fn test_regex_image() {
    let cap: Vec<_> = Regex::new(REGEX_IMAGE)
        .unwrap()
        .captures_iter(r"![image1](https://image1.com) ![image2](https://image2.com)")
        .collect();

    let first = cap.first().unwrap();
    assert_eq!(&first[0], "![image1](https://image1.com)");
    assert_eq!(&first["alt"], "image1");
    assert_eq!(&first["url"], "https://image1.com");

    let second = cap.get(1).unwrap();
    assert_eq!(&second[0], "![image2](https://image2.com)");
    assert_eq!(&second["alt"], "image2");
    assert_eq!(&second["url"], "https://image2.com");
}
