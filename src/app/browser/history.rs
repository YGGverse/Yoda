//! Browser history dialog

use super::Profile;
use crate::app::browser::window::action::{Action as WindowAction, Position};
use adw::{
    ActionRow, PreferencesGroup, PreferencesPage,
    prelude::{
        ActionRowExt, AdwDialogExt, ExpanderRowExt, PreferencesDialogExt, PreferencesGroupExt,
        PreferencesPageExt,
    },
};
use gtk::glib::{DateTime, GString, Uri, UriFlags, gformat};
use indexmap::IndexMap;
use std::rc::Rc;

pub struct Event {
    pub time: DateTime,
    pub count: usize,
}

struct Record {
    event: Event,
    request: GString,
    title: Option<GString>,
}

pub trait History {
    fn history(window_action: &Rc<WindowAction>, profile: &Rc<Profile>) -> Self;
}

impl History for adw::PreferencesDialog {
    fn history(window_action: &Rc<WindowAction>, profile: &Rc<Profile>) -> Self {
        let mut visited: IndexMap<GString, Vec<Record>> = IndexMap::new();
        // @TODO recently closed

        for history in profile.history.recently_opened(None) {
            match Uri::parse(&history.request, UriFlags::NONE) {
                Ok(uri) => visited
                    .entry(match uri.host() {
                        Some(host) => host,
                        None => uri.to_str(),
                    })
                    .or_default()
                    .push(Record {
                        event: Event {
                            time: history.opened.time,
                            count: history.opened.count,
                        },
                        request: history.request,
                        title: history.title,
                    }),
                Err(_) => continue, // @TODO
            }
        }

        let d = adw::PreferencesDialog::builder()
            .search_enabled(true)
            .title("History")
            .build();

        d.add(&{
            let p = PreferencesPage::builder()
                .icon_name("document-open-recent-symbolic")
                .title("Recently visited")
                .build();

            for (group, records) in visited {
                p.add(&{
                    let g = PreferencesGroup::new();
                    g.add(&{
                        let e = adw::ExpanderRow::builder()
                            .enable_expansion(true)
                            .expanded(false)
                            .subtitle(
                                records
                                    .iter()
                                    .max_by_key(|r| r.event.time.to_unix())
                                    .unwrap()
                                    .event
                                    .time
                                    .format_iso8601()
                                    .unwrap(),
                            )
                            .title_selectable(true)
                            .title(group)
                            .build();

                        for record in records {
                            e.add_row(&{
                                let a = ActionRow::builder()
                                    .activatable(true)
                                    // @TODO use button widget to open the links on click
                                    //.title_selectable(true)
                                    .title(match record.title {
                                        Some(title) => title,
                                        None => gformat!(
                                            "{} ({})",
                                            record.event.time.format_iso8601().unwrap(),
                                            record.event.count
                                        ),
                                    })
                                    .subtitle_selectable(true)
                                    .subtitle(&*record.request)
                                    .build();

                                a.connect_activated({
                                    let a = window_action.clone();
                                    let d = d.clone();
                                    move |_| {
                                        a.append.activate_stateful_once(
                                            Position::After,
                                            Some(record.request.to_string()),
                                            false,
                                            true,
                                            true,
                                            true,
                                        );
                                        d.close();
                                    }
                                });
                                a
                            })
                        }
                        e
                    });
                    g
                });
            }
            p
        });
        d
    }
}
