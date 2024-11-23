#[derive(Debug)]
pub enum Error {
    Clear,
    NotFound(i64),
    Overwrite(i64),
    Unexpected,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Clear => "Could not cleanup memory index".to_string(),
            Self::NotFound(key) => {
                format!("Record `{key}` not found in memory index")
            }
            Self::Overwrite(key) => {
                format!("Overwrite attempt for existing record `{key}`")
            }
            Self::Unexpected => "Unexpected error".to_string(),
        }
    }
}
