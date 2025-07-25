#[derive(Clone)]
pub struct Item {
    pub id: i64,
    pub request: String,
    pub time: gtk::glib::DateTime,
    pub title: Option<String>,
}
