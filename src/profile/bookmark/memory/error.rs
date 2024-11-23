#[derive(Debug)]
pub enum Error {
    Overwrite(String),
    Unexpected,
}

impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::Overwrite(key) => {
                format!("Overwrite attempt for existing record `{key}`")
            }
            Self::Unexpected => "Unexpected error".to_string(),
        }
    }
}
