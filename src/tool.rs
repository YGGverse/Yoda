//! Some shared helpers collection

// Global dependencies
use gtk::glib::{DateTime, GString};

/// Format bytes to KB/MB/GB presentation
pub fn format_bytes(value: usize) -> String {
    const KB: f32 = 1024.0;
    const MB: f32 = KB * KB;
    const GB: f32 = MB * KB;

    let f = value as f32;

    if f < KB {
        format!(
            "{value} {}",
            plurify::ns(value, &["byte", "bytes", "bytes"])
        )
    } else if f < MB {
        format!("{:.2} KB", f / KB)
    } else if f < GB {
        format!("{:.2} MB", f / MB)
    } else {
        format!("{:.2} GB", f / GB)
    }
}

/// Format given [DateTime](https://docs.gtk.org/glib/struct.DateTime.html)
pub fn format_time(t: &DateTime) -> GString {
    t.format_iso8601().unwrap() // @TODO handle?
}

/// Get current [DateTime](https://docs.gtk.org/glib/struct.DateTime.html)
pub fn now() -> DateTime {
    DateTime::now_local().unwrap() // @TODO handle?
}
