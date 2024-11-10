//! Global actions config

use gtk::glib::VariantTy;

// app/browser/widget.rs
pub const APP_BROWSER_WIDGET: &str = "app_browser_widget";

// group | action | variant
pub const APP_BROWSER_WIDGET_ABOUT: (&str, &str, Option<&VariantTy>) =
    (APP_BROWSER_WIDGET, "about", None);

pub const APP_BROWSER_WIDGET_CLOSE: (&str, &str, Option<&VariantTy>, &[&str]) =
    (APP_BROWSER_WIDGET, "close", None, &["<Primary>Escape"]);

// group | action | variant | accels
pub const APP_BROWSER_WIDGET_DEBUG: (&str, &str, Option<&VariantTy>, &[&str]) =
    (APP_BROWSER_WIDGET, "debug", None, &["<Primary>i"]);
