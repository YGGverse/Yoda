use std::sync::Arc;

pub struct Database {
    pub connection: Arc<sqlite::Connection>,
}

impl Database {
    fn init(&self) {}
    fn save(&self) {}
    fn restore(&self) {}
}
