mod feature;
mod gemini;

use super::{Client, Response};
use feature::Feature;
use gtk::{
    gio::Cancellable,
    glib::{Uri, UriFlags},
};

/// Single `Request` API for multiple `Client` drivers
pub enum Request {
    Gemini {
        feature: Feature,
        referrer: Vec<Self>,
        uri: Uri,
    },
    Titan(Uri),
}

impl Request {
    // Actions

    /// Process request by routed driver
    pub fn route(
        client: &Client,
        query: &str,
        referrer: Option<Vec<Self>>,
        cancellable: Cancellable,
        callback: impl FnOnce(Response) + 'static,
    ) {
        let (feature, request) = Feature::parse(query);

        match Uri::parse(request, UriFlags::NONE) {
            Ok(uri) => match uri.scheme().as_str() {
                "gemini" => gemini::route(client, feature, uri, referrer, cancellable, callback),
                "titan" => todo!(),
                _ => callback(Response::Redirect(
                    todo!(), //super::response::Redirect::Foreground(()),
                )),
            },
            Err(_) => todo!(),
        }
    }

    // Getters

    /// Get reference to `Self` [URI](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn as_uri(&self) -> &Uri {
        match self {
            Self::Gemini {
                feature: _,
                referrer: _,
                uri,
            }
            | Self::Titan(uri) => &uri,
        }
    }
}
