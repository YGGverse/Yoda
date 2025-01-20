mod error;
mod feature;
mod gemini;
mod search;

use gemini::Gemini;

use super::{Client, Response};
pub use error::Error;
use feature::Feature;
use gtk::{
    gio::Cancellable,
    glib::{Uri, UriFlags},
};

/// Single `Request` API for multiple `Client` drivers
pub enum Request {
    Gemini(Gemini, Feature),
    Titan {
        referrer: Option<Box<Self>>,
        uri: Uri,
    }, // @TODO deprecated
}

impl Request {
    // Constructors

    /// Create new `Self` from featured string
    pub fn parse(query: &str) -> Result<Self, Error> {
        let (feature, request) = Feature::parse(query);

        match Uri::parse(request, UriFlags::NONE) {
            Ok(uri) => Self::from_uri(uri, feature),
            Err(e) => Err(Error::Glib(e)),
        }
    }

    /// Create new `Self` from [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    pub fn from_uri(uri: Uri, feature: Feature) -> Result<Self, Error> {
        match uri.scheme().as_str() {
            "gemini" => Ok(Self::Gemini(
                Gemini {
                    uri,
                    referrer: None,
                },
                feature,
            )),
            "titan" => todo!(),
            _ => Err(Error::Unsupported),
        }
    }

    /// Create new `Self` as the redirection query to default search provider
    /// @TODO
    // * implement DNS lookup before apply this option
    // * make search provider optional
    // * validate request len by gemini specifications
    pub fn search(query: &str) -> Self {
        Self::from_uri(search::tgls(query), Feature::Default).unwrap() // no handler as unexpected
    }

    /// Create new `Self` using DNS async resolver (slow method)
    /// * useful for scheme-less requests, before apply search redirect
    pub fn lookup(
        query: &str,
        cancellable: Option<&Cancellable>,
        callback: impl FnOnce(Result<Self, Error>) + 'static,
    ) {
        use gtk::{
            gio::{NetworkAddress, Resolver},
            prelude::{NetworkAddressExt, ResolverExt},
        };

        const DEFAULT_SCHEME: &str = "gemini";
        const DEFAULT_PORT: u16 = 1965;
        const TIMEOUT: u32 = 250; // ms

        let query = query.trim();

        match Uri::parse(query, UriFlags::NONE) {
            Ok(uri) => callback(Self::from_uri(uri, Feature::Default)),
            Err(_) => {
                // try default scheme suggestion
                let suggestion = format!("{DEFAULT_SCHEME}://{query}");

                let resolver = Resolver::default();
                resolver.set_timeout(TIMEOUT);

                match NetworkAddress::parse_uri(&suggestion, DEFAULT_PORT) {
                    Ok(connectable) => resolver.lookup_by_name_async(
                        &connectable.hostname(),
                        cancellable,
                        move |resolve| {
                            callback(if resolve.is_ok() {
                                Self::parse(&suggestion)
                            } else {
                                Ok(Self::search(&suggestion))
                            })
                        },
                    ),
                    Err(_) => callback(Ok(Self::search(&suggestion))),
                }
            }
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
            Self::Gemini(this, feature) => this.handle(client, cancellable, callback),
            Self::Titan { .. } => todo!(),
        }
    }
}
