pub enum PrimaryIcon<'a> {
    Download {
        name: &'a str,
        tooltip: &'a str,
    },
    Gemini {
        name: &'a str,
        tooltip: (&'a str, &'a str),
    },
    Search {
        name: &'a str,
        tooltip: &'a str,
    },
    Source {
        name: &'a str,
        tooltip: &'a str,
    },
    Titan {
        name: &'a str,
        tooltip: &'a str,
    },
}

pub fn from(request: &str) -> PrimaryIcon {
    if request.starts_with("download:") {
        return PrimaryIcon::Download {
            name: "document-save-symbolic",
            tooltip: "Download",
        };
    }

    if request.starts_with("source:") {
        return PrimaryIcon::Source {
            name: "applications-system-symbolic",
            tooltip: "Source view",
        };
    }

    if request.starts_with("gemini:") {
        return PrimaryIcon::Gemini {
            name: "channel-secure-symbolic",
            tooltip: ("Guest session", "User session"),
        };
    }

    if request.starts_with("titan:") {
        return PrimaryIcon::Titan {
            name: "document-send-symbolic",
            tooltip: "Titan input",
        };
    }

    PrimaryIcon::Search {
        name: "system-search-symbolic",
        tooltip: "Search",
    }
}
