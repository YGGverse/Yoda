use std::sync::Arc;

pub struct Browser {
    connection: Arc<sqlite::Connection>,
}

impl Browser {
    fn init(&self) {}
    fn save(&self) {}
    fn restore(&self) {}
}

pub fn new(connection: Arc<sqlite::Connection>) -> Browser {
    let this = Browser { connection };
    this.init();
    this
}
