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

    /// Create new `Self` from featured string
    pub fn parse(query: &str, referrer: Option<Vec<Self>>) -> Result<Self, Error> {
        let (feature, request) = Feature::parse(query);

        match Uri::parse(request, UriFlags::NONE) {
            Ok(uri) => Self::from_uri(uri, Some(feature), referrer),
            Err(e) => Err(Error::Glib(e)),
        }
    }

    /// Create new `Self` from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn from_uri(
        uri: Uri,
        feature: Option<Feature>,
        referrer: Option<Vec<Self>>,
    ) -> Result<Self, Error> {
        match uri.scheme().as_str() {
            "gemini" => Ok(Self::Gemini {
                feature: feature.unwrap_or_default(),
                referrer: referrer.unwrap_or_default(),
                uri,
            }),
            "titan" => Ok(Self::Titan(uri)),
            _ => Err(Error::Unsupported),
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
            } => gemini::send(client, feature, uri, referrer, cancellable, callback),
            Self::Titan(_) => todo!(),
        }
    }

    // Getters

    /// Get reference to `Self` [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn as_uri(&self) -> &Uri {
        match self {
            Self::Gemini {
                feature: _,
                referrer: _,
                uri,
            }
            | Self::Titan(uri) => uri,
        }
    }
}
