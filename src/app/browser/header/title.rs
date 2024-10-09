use adw::WindowTitle;

const DEFAULT_TITLE: &str = "Yoda"; // @TODO
const DEFAULT_SUBTITLE: &str = "";

pub struct Title {
    gobject: WindowTitle,
}

impl Title {
    // Construct
    pub fn new() -> Self {
        Self {
            gobject: WindowTitle::new(DEFAULT_TITLE, DEFAULT_SUBTITLE),
        }
    }

    // Actions
    pub fn update(&self, title: Option<&str>, subtitle: Option<&str>) {
        // Update title
        let mut name = Vec::new();

        if let Some(value) = title {
            if !value.is_empty() {
                name.push(value);
            }
        }

        name.push(DEFAULT_TITLE);

        self.gobject.set_title(&name.join(" - "));

        // Update subtitle
        self.gobject.set_subtitle(&match subtitle {
            Some(value) => value,
            None => "", // @TODO
        });
    }

    // Getters
    pub fn gobject(&self) -> &WindowTitle {
        &self.gobject
    }
}
