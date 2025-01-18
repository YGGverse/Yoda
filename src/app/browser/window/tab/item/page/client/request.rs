pub mod feature;
pub use feature::Feature;

use gtk::{
    gio::Cancellable,
    glib::{Priority, Uri},
};

/// Request data wrapper for `Client`
#[derive(Clone)]
pub struct Request {
    pub feature: Feature,
    /// Requests chain in order to process redirection rules
    pub referrer: Vec<Request>,
}

impl Request {
    // Constructors

    /// Build new `Self`
    pub fn build(
        query: &str,
        referrer: Option<Vec<Request>>,
        cancellable: Cancellable,
        priority: Priority,
    ) -> Self {
        Self {
            feature: Feature::build(query, cancellable, priority),
            referrer: referrer.unwrap_or_default(),
        }
    }

    // Getters

    /// Copy `Self` to new `referrer` vector
    pub fn to_referrer(&self) -> Vec<Request> {
        let mut referrer = self.referrer.to_vec();
        referrer.push(self.clone());
        referrer
    }

    pub fn uri(&self) -> Option<&Uri> {
        self.feature.uri()
    }
}
