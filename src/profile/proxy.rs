mod database;
mod ignore;
mod rule;

use anyhow::Result;
use database::Database;
use gtk::{
    gio::{ProxyResolver, SimpleProxyResolver},
    glib::DateTime,
};
use ignore::Ignore;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rule::Rule;
use std::cell::RefCell;

pub struct Proxy {
    database: Database,
    ignore: RefCell<Vec<Ignore>>,
    rule: RefCell<Vec<Rule>>,
}

impl Proxy {
    // Constructors

    pub fn init(database_pool: &Pool<SqliteConnectionManager>, profile_id: i64) -> Result<Self> {
        let database = Database::init(database_pool, profile_id);

        let ignores = database.ignores()?;
        let ignore = RefCell::new(Vec::with_capacity(ignores.len()));

        {
            // build in-memory index...
            let mut b = ignore.borrow_mut();
            for i in ignores {
                b.push(Ignore {
                    is_enabled: i.is_enabled,
                    host: i.host,
                });
            }
        }

        let rules = database.rules()?;
        let rule = RefCell::new(Vec::with_capacity(rules.len()));

        {
            // build in-memory index...
            let mut b = rule.borrow_mut();
            for r in rules {
                b.push(Rule {
                    id: Some(r.id),
                    time: r.time,
                    is_enabled: r.is_enabled,
                    priority: r.priority,
                    request: r.request,
                    url: r.url,
                });
            }
        }

        Ok(Self {
            database,
            ignore,
            rule,
        })
    }

    // Actions

    pub fn add_rule(
        &self,
        id: Option<i64>,
        is_enabled: bool,
        priority: i32,
        request: String,
        url: String,
        time: DateTime,
    ) {
        let mut rules = self.rule.borrow_mut();
        rules.push(Rule {
            id,
            time,
            is_enabled,
            priority,
            request,
            url,
        }) // @TODO validate?
    }

    pub fn clear(&self) {
        self.ignore.borrow_mut().clear();
        self.rule.borrow_mut().clear();
    }

    pub fn save(&self) -> Result<()> {
        let rules = self.rule.take();
        let mut keep_id = Vec::with_capacity(rules.len());
        for rule in rules {
            keep_id.push(self.database.persist_rule(
                rule.id,
                rule.time,
                rule.is_enabled,
                rule.priority,
                rule.request,
                rule.url,
            )?);
        }
        self.database.clean_rules(keep_id)?;
        Ok(())
    }

    // Getters

    pub fn rules(&self) -> Vec<Rule> {
        self.rule.borrow().iter().cloned().collect()
    }

    pub fn matches(&self, request: &str) -> Option<ProxyResolver> {
        for rule in self.rule.borrow().iter().filter(|r| r.is_enabled) {
            if gtk::glib::Regex::match_simple(
                &rule.request,
                request,
                gtk::glib::RegexCompileFlags::DEFAULT,
                gtk::glib::RegexMatchFlags::DEFAULT,
            ) {
                return Some(SimpleProxyResolver::new(
                    Some(&rule.url),
                    self.ignore
                        .borrow()
                        .iter()
                        .filter_map(|i| {
                            if i.is_enabled {
                                Some(i.host.clone())
                            } else {
                                None
                            }
                        })
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
    database::init(tx)?;

    // Delegate migration to childs
    // nothing yet...

    // Success
    Ok(())
}
