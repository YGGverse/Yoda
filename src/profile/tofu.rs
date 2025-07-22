mod certificate;
mod database;

use anyhow::Result;
use certificate::Certificate;
use database::Database;
use gtk::{gio::TlsCertificate, glib::Uri};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sourceview::prelude::TlsCertificateExt;
use sqlite::Transaction;
use std::sync::RwLock;

/// TOFU wrapper for the Gemini protocol
///
/// https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation
pub struct Tofu {
    database: Database,
    memory: RwLock<Vec<Certificate>>,
}

impl Tofu {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id);

        let records = database.records()?;
        let memory = RwLock::new(Vec::with_capacity(records.len()));

        {
            // build in-memory index...
            let mut m = memory.write().unwrap();
            for r in records {
                m.push(Certificate::from_db(Some(r.id), &r.pem, r.time)?)
            }
        }

        Ok(Self { database, memory })
    }

    // Actions

    pub fn add(&self, tls_certificate: TlsCertificate) -> Result<()> {
        self.memory
            .write()
            .unwrap()
            .push(Certificate::from_tls_certificate(tls_certificate)?);
        Ok(())
    }

    pub fn server_certificates(&self, uri: &Uri) -> Option<Vec<TlsCertificate>> {
        fn f(subject_name: &str) -> String {
            subject_name
                .trim_start_matches("CN=")
                .trim_start_matches('*')
                .trim_matches('.')
                .to_lowercase()
        }
        if let Some(h) = uri.host() {
            let k = f(&h);
            let m = self.memory.read().unwrap();
            let b: Vec<TlsCertificate> = m
                .iter()
                .filter_map(|certificate| {
                    let tls_certificate = certificate.tls_certificate();
                    if k.ends_with(&f(&tls_certificate.subject_name().unwrap())) {
                        Some(tls_certificate.clone())
                    } else {
                        None
                    }
                })
                .collect();
            if !b.is_empty() {
                return Some(b);
            }
        }
        None
    }

    /// Save in-memory index to the permanent database (on app close)
    pub fn save(&self) -> Result<()> {
        for c in self.memory.read().unwrap().iter() {
            if c.id().is_none() {
                self.database.add(c.time(), &c.pem())?;
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
