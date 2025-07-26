#[derive(Clone)]
pub struct Memory {
    pub host: String,
    pub id: Option<i64>,
    pub is_enabled: bool,
    pub time: gtk::glib::DateTime,
}
