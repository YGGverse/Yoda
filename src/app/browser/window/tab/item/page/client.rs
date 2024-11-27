/// Single client instance holder for `Page` object.
///
/// unlike new client instance init on every page open,
/// this solution provide additional client-side features
/// e.g. session cache management on certificate change in runtime
pub struct Client {
    pub gemini: gemini::Client,
    // other supported clients here..
}

impl Client {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            gemini: gemini::Client::new(),
        }
    }
}
