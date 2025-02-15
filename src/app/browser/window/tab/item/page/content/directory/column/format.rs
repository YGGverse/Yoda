use gtk::{gio::FileInfo, glib::GString};

const DEFAULT: &str = "-";

pub trait Format {
    fn format_content_type(&self) -> GString;
    fn format_date_time(&self) -> GString;
}

impl Format for FileInfo {
    fn format_content_type(&self) -> GString {
        match self.content_type() {
            Some(content_type) => {
                let display_name = self.display_name();
                if content_type == "text/plain" {
                    if display_name.ends_with(".gmi") || display_name.ends_with(".gemini") {
                        "text/gemini".into()
                    } else {
                        content_type
                    }
                } else {
                    content_type
                }
            }
            None => DEFAULT.into(),
        }
    }
    fn format_date_time(&self) -> GString {
        match self.creation_date_time() {
            Some(date_time) => date_time
                .format("%Y.%m.%d %H:%M:%S") // @TODO optional
                .unwrap_or(DEFAULT.into()),
            None => DEFAULT.into(),
        }
    }
}
