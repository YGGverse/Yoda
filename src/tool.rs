//! Some shared helpers collection

pub trait Format {
    /// Format bytes to KB/MB/GB presentation
    fn bytes(self) -> String;
}

impl Format for usize {
    fn bytes(self) -> String {
        use plurify::*;

        const KB: f32 = 1024.0;
        const MB: f32 = KB * KB;
        const GB: f32 = MB * KB;

        let f = self as f32;

        if f < KB {
            format!("{self} {}", self.plurify(&["byte", "bytes", "bytes"]))
        } else if f < MB {
            format!("{:.2} KB", f / KB)
        } else if f < GB {
            format!("{:.2} MB", f / MB)
        } else {
            format!("{:.2} GB", f / GB)
        }
    }
}

/// Helper function, extract readable title from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
/// * this feature wants to be improved @TODO
pub fn uri_to_title(uri: &gtk::glib::Uri) -> gtk::glib::GString {
    let path = uri.path();

    if path.split('/').last().unwrap_or_default().is_empty() {
        match uri.host() {
            Some(host) => host,
            None => "Untitled".into(),
        }
    } else {
        path
    }
}
