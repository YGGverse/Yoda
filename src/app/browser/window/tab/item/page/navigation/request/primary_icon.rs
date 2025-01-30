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
        tooltip: (&'a str, &'a str),
    },
}

pub fn from(request: &str) -> PrimaryIcon {
    let prefix = request.to_lowercase();

    if prefix.starts_with("download:") {
        return PrimaryIcon::Download {
            name: "document-save-symbolic",
            tooltip: "Download",
        };
    }

    if prefix.starts_with("source:") {
        return PrimaryIcon::Source {
            name: "applications-system-symbolic",
            tooltip: "Source view",
        };
    }

    if prefix.starts_with("gemini:") {
        return PrimaryIcon::Gemini {
            name: "channel-secure-symbolic",
            tooltip: ("Guest session", "User session"),
        };
    }

    if prefix.starts_with("titan:") {
        return PrimaryIcon::Titan {
            name: "document-send-symbolic",
            tooltip: ("Guest titan input", "User titan input"),
        };
    }

    PrimaryIcon::Search {
        name: "system-search-symbolic",
        tooltip: "Choose default search provider",
    }
}
