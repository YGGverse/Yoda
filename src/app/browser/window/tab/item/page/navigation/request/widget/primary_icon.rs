pub enum PrimaryIcon<'a> {
    Download {
        icon_name: &'a str,
        tooltip_text: &'a str,
    },
    Gemini {
        icon_name: &'a str,
        tooltip_text: (&'a str, &'a str),
    },
    Search {
        icon_name: &'a str,
        tooltip_text: &'a str,
    },
    Source {
        icon_name: &'a str,
        tooltip_text: &'a str,
    },
}

pub fn from(request: &str) -> PrimaryIcon {
    if request.starts_with("download:") {
        return PrimaryIcon::Download {
            icon_name: "document-save-symbolic",
            tooltip_text: "Download",
        };
    }

    if request.starts_with("source:") {
        return PrimaryIcon::Source {
            icon_name: "applications-system-symbolic",
            tooltip_text: "Source view",
        };
    }

    if request.starts_with("gemini:") {
        return PrimaryIcon::Gemini {
            icon_name: "channel-secure-symbolic",
            tooltip_text: ("Guest session", "User session"),
        };
    }

    PrimaryIcon::Search {
        icon_name: "system-search-symbolic",
        tooltip_text: "Search",
    }
}
