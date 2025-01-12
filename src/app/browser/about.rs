use adw::{prelude::AdwDialogExt, AboutDialog};
use gtk::{prelude::IsA, License};

pub struct About {
    gobject: AboutDialog,
}

impl Default for About {
    fn default() -> Self {
        Self::new()
    }
}

impl About {
    // Construct
    pub fn new() -> Self {
        // Collect debug info
        let debug = &[
            format!(
                "Adwaita {}.{}.{}",
                adw::major_version(),
                adw::minor_version(),
                adw::micro_version()
            ),
            format!(
                "Gtk {}.{}.{}",
                gtk::major_version(),
                gtk::minor_version(),
                gtk::micro_version()
            ),
            format!(
                "GtkSourceView {}.{}.{}",
                sourceview::major_version(),
                sourceview::minor_version(),
                sourceview::micro_version()
            ),
            format!("SQLite {}", sqlite::version()),
            // @TODO
        ];

        // Init gobject
        let gobject = AboutDialog::builder()
            .application_name(env!("CARGO_PKG_NAME"))
            .debug_info(debug.join("\n"))
            .developer_name(env!("CARGO_PKG_DESCRIPTION"))
            .issue_url(env!("CARGO_PKG_REPOSITORY"))
            .license_type(License::MitX11)
            .version(env!("CARGO_PKG_VERSION"))
            .build();

        // Return new struct
        Self { gobject }
    }

    // Actions
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.gobject.present(parent);
    }
}
