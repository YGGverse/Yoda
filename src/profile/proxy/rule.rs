#[derive(Clone)]
pub struct Rule {
    pub id: Option<i64>,
    pub is_enabled: bool,
    pub priority: i32,
    pub request: String,
    pub time: gtk::glib::DateTime,
    pub url: String,
}
