#[derive(Debug)]
pub enum Error {
    Clear,
    Overwrite(String),
    Unexpected,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Clear => format!("Could not cleanup memory index"),
            Self::Overwrite(key) => {
                format!("Overwrite attempt for existing record `{key}`")
            }
            Self::Unexpected => format!("Unexpected error"),
        }
    }
}
