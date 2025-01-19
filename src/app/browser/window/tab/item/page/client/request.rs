mod error;
mod feature;
mod gemini;
mod search;

use super::{Client, Response};
pub use error::Error;
use feature::Feature;
use gtk::{
    gio::Cancellable,
    glib::{Uri, UriFlags},
};

/// Single `Request` API for multiple `Client` drivers
pub enum Request {
    Gemini {
        feature: Feature,
        referrer: Option<Box<Self>>,
        uri: Uri,
    },
    Titan {
        referrer: Option<Box<Self>>,
        uri: Uri,
    },
}

impl Request {
    // Constructors

    /// Create new `Self` from featured string
    pub fn parse(query: &str, referrer: Option<Self>) -> Result<Self, Error> {
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
        referrer: Option<Self>,
    ) -> Result<Self, Error> {
        match uri.scheme().as_str() {
            "gemini" => Ok(Self::Gemini {
                feature: feature.unwrap_or_default(),
                referrer: referrer.map(Box::new),
                uri,
            }), // @TODO validate request len by constructor
            "titan" => Ok(Self::Titan {
                referrer: referrer.map(Box::new),
                uri,
            }),
            _ => Err(Error::Unsupported),
        }
    }

    /// Create new `Self` as the redirection query to default search provider
    /// @TODO
    // * implement DNS lookup before apply this option
    // * make search provider optional
    // * validate request len by gemini specifications
    pub fn search(query: &str) -> Self {
        Self::from_uri(search::tgls(query), None, None).unwrap() // no handler as unexpected
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

        const DEFAULT_SCHEME: &str = "gemini://";
        const DEFAULT_PORT: u16 = 1965;
        const TIMEOUT: u32 = 1000;

        let request = match query.trim().strip_prefix(DEFAULT_SCHEME) {
            Some(postfix) => format!("{DEFAULT_SCHEME}{postfix}"),
            None => query.to_string(),
        };

        let resolver = Resolver::default();
        resolver.set_timeout(TIMEOUT);

        match NetworkAddress::parse_uri(&request, DEFAULT_PORT) {
            Ok(connectable) => resolver.lookup_by_name_async(
                &connectable.hostname(),
                cancellable,
                move |resolve| {
                    callback(if resolve.is_ok() {
                        match Uri::parse(&request, UriFlags::NONE) {
                            Ok(uri) => Self::from_uri(uri, None, None),
                            Err(e) => Err(Error::Glib(e)),
                        }
                    } else {
                        Ok(Self::search(&request))
                    })
                },
            ),
            Err(_) => callback(Ok(Self::search(&request))), // @TODO not completed yet, fix invalid URI issue
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
        match &self {
            Self::Gemini { .. } => gemini::request(client, self, cancellable, callback),
            Self::Titan { .. } => todo!(),
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
            | Self::Titan { referrer: _, uri } => uri,
        }
    }

    /// Get `Feature` reference for `Self`
    pub fn feature(&self) -> &Feature {
        match self {
            Request::Gemini { feature, .. } => feature,
            Request::Titan { .. } => &Feature::Default,
        }
    }

    /// Recursively count referrers of `Self`
    /// * useful to apply redirection rules by protocol driver selected
    pub fn referrers(&self) -> usize {
        let count = match self {
            Request::Gemini { referrer, .. } => referrer,
            Request::Titan { referrer, .. } => referrer,
        }
        .as_ref()
        .map_or(0, |request| request.referrers());
        1 + count
    }
}
