mod ignore;
mod misc;
mod rule;

use anyhow::Result;
use gtk::gio::{ProxyResolver, SimpleProxyResolver};
use ignore::Ignore;
use misc::Misc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rule::Rule;

pub struct Proxy {
    pub ignore: Ignore,
    pub rule: Rule,
    pub misc: Misc,
}

impl Proxy {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        Ok(Self {
            ignore: Ignore::init(database_pool, profile_id)?,
            misc: Misc::init(database_pool, profile_id)?,
            rule: Rule::init(database_pool, profile_id)?,
        })
    }

    // Actions

    pub fn save(&self) -> Result<()> {
        self.ignore.save()?;
        self.misc.save()?;
        self.rule.save()?;
        Ok(())
    }

    // Getters

    pub fn matches(&self, request: &str) -> Option<ProxyResolver> {
        for rule in self.rule.enabled() {
            if gtk::glib::Regex::match_simple(
                &rule.request,
                request,
                gtk::glib::RegexCompileFlags::DEFAULT,
                gtk::glib::RegexMatchFlags::DEFAULT,
            ) {
                return Some(SimpleProxyResolver::new(
                    Some(&rule.url),
                    self.ignore
                        .enabled()
                        .into_iter()
                        .map(|i| i.host)
                        .collect::<Vec<String>>(),
                ));
            }
        }
        None
    }
}

// Tools

pub fn migrate(tx: &sqlite::Transaction) -> Result<()> {
    // Migrate self components
    // nothing yet..

    // Delegate migration to childs
    ignore::migrate(tx)?;
    misc::migrate(tx)?;
    rule::migrate(tx)?;

    Ok(())
}
