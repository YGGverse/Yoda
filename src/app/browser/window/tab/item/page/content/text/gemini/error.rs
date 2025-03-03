pub enum Error {
    Multiline(super::Gemini),
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Self::Multiline(_) => {
                "Invalid multiline markup! Gemtext format partially ignored.".to_string()
            }
        }
    }
}
