mod certificate;
mod database;

use anyhow::Result;
use certificate::Certificate;
use database::Database;
use gtk::{
    gio::TlsCertificate,
    glib::{GString, Uri},
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap};

/// TOFU wrapper for the Gemini protocol
///
/// https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation
pub struct Tofu {
    database: Database,
    memory: RefCell<HashMap<(GString, i32), Certificate>>,
}

impl Tofu {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id);

        let records = database.records()?;
        let memory = RefCell::new(HashMap::with_capacity(records.len()));

        {
            // build in-memory index...
            let mut m = memory.borrow_mut();
            for r in records {
                if m.insert(
                    (r.host.into(), r.port),
                    Certificate::from_db(Some(r.id), &r.pem, r.time)?,
                )
                .is_some()
                {
                    panic!() // expect unique address
                }
            }
        }

        Ok(Self { database, memory })
    }

    // Actions

    pub fn add(
        &self,
        uri: &Uri,
        default_port: i32,
        tls_certificate: TlsCertificate,
    ) -> Result<bool> {
        match uri.host() {
            Some(host) => Ok(self
                .memory
                .borrow_mut()
                .insert(
                    (host, port(uri.port(), default_port)),
                    Certificate::from_tls_certificate(tls_certificate)?,
                )
                .is_none()),
            None => Ok(false),
        }
    }

    pub fn server_certificate(&self, uri: &Uri, default_port: i32) -> Option<TlsCertificate> {
        uri.host().and_then(|host| {
            self.memory
                .borrow()
                .get(&(host, port(uri.port(), default_port)))
                .map(|c| c.tls_certificate().clone())
        })
    }

    /// Save in-memory index to the permanent database (on app close)
    pub fn save(&self) -> Result<()> {
        for ((host, port), certificate) in self.memory.borrow_mut().drain() {
            if certificate.id().is_none() {
                self.database
                    .add(host.into(), port, certificate.time(), &certificate.pem())?;
            }
        }
        Ok(())
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    // nothing yet...

    // Success
    Ok(())
}

fn port(port: i32, default_port: i32) -> i32 {
    if port.is_positive() {
        port
    } else {
        default_port
    }
}
