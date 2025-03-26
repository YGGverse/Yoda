/// Common redirection type enumeration for different protocol drivers
pub enum Method {
    Permanent,
    Temporary,
}

impl Method {
    pub fn icon_name(&self) -> &str {
        match self {
            Self::Permanent => "network-transmit-symbolic",
            Self::Temporary => "network-transmit-receive-symbolic",
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Permanent => write!(f, "Permanent"),
            Self::Temporary => {
                write!(f, "Temporary")
            }
        }
    }
}
