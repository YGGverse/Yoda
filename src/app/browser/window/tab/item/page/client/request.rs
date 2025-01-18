mod error;
mod feature;
mod gemini;

use super::{Client, Response};
use error::Error;
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
    // Constructors

    /// Create new `Self` from string
    pub fn parse(query: &str, referrer: Option<Vec<Self>>) -> Result<Self, Error> {
        let (feature, request) = Feature::parse(query);

        match Uri::parse(request, UriFlags::NONE) {
            Ok(uri) => match uri.scheme().as_str() {
                "gemini" => Ok(Self::Gemini {
                    feature,
                    referrer: referrer.unwrap_or_default(),
                    uri,
                }),
                "titan" => Ok(Self::Titan(uri)),
                _ => Err(Error::Unsupported),
            },
            Err(e) => Err(Error::Glib(e)),
        }
    }

    // Actions

    /// Handle `Self` request
    pub fn handle(
        self,
        client: &Client,
        cancellable: Cancellable,
        callback: impl FnOnce(Response) + 'static,
    ) {
        match self {
            Self::Gemini {
                feature,
                referrer,
                uri,
            } => gemini::route(
                client,
                feature.clone(),
                uri.clone(),
                referrer,
                cancellable,
                callback,
            ),
            Self::Titan(_) => todo!(),
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
