use gtk::Entry;

pub fn new() -> Entry {
    Entry::builder()
        .placeholder_text("URL or search term...")
        .hexpand(true)
        .progress_pulse_step(0.1)
        .build()
}
