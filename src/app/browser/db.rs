/* @TODO
use std::sync::Arc;

pub struct Browser {
    connection: Arc<sqlite::Connection>,
}

impl Browser {
    // Construct new browser DB (connection)
    pub fn new(connection: Arc<sqlite::Connection>) -> Browser {
        let this = Self { connection };
        this.init();
        this
    }

    // Create browser table in DB if not exist yet
    fn init(&self) {}

    // Save active browser session to DB
    fn save(&self) {}

    // Restore previous browser session from DB
    fn restore(&self) {}
}*/
