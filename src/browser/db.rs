use std::sync::Arc;

pub struct Browser {
    pub connection: Arc<sqlite::Connection>,
}

impl Browser {
    fn init(&self) {}
    fn save(&self) {}
    fn restore(&self) {}
}
