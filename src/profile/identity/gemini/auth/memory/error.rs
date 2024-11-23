#[derive(Debug)]
pub enum Error {
    Clear,
    Overwrite(String),
    Unexpected,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Clear => "Could not cleanup memory index".to_string(),
            Self::Overwrite(key) => {
                format!("Overwrite attempt for existing record `{key}`")
            }
            Self::Unexpected => "Unexpected error".to_string(),
        }
    }
}
