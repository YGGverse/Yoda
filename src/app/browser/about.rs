pub trait About {
    fn about() -> Self;
}

impl About for adw::AboutDialog {
    fn about() -> Self {
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

        adw::AboutDialog::builder()
            .application_icon("io.github.yggverse.Yoda")
            .application_name(env!("CARGO_PKG_NAME"))
            .debug_info(debug.join("\n"))
            .developer_name(env!("CARGO_PKG_DESCRIPTION"))
            .issue_url(env!("CARGO_PKG_REPOSITORY"))
            .license_type(gtk::License::MitX11)
            .version(env!("CARGO_PKG_VERSION"))
            .build()
    }
}
