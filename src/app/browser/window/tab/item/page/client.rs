pub struct Client {
    pub gemini: gemini::Client,
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
