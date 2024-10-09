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
    pub fn update(&self, title: &str, subtitle: &str) {
        // Update title
        let mut parts = Vec::new();

        if !title.is_empty() {
            parts.push(title);
        }

        parts.push(DEFAULT_TITLE);

        self.gobject.set_title(&parts.join(" - "));

        // Update subtitle
        self.gobject.set_subtitle(subtitle);
    }

    // Getters
    pub fn gobject(&self) -> &WindowTitle {
        &self.gobject
    }
}
