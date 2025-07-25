//! Browser bookmarks dialog

use super::Profile;
use crate::app::browser::window::action::{Action as WindowAction, Position};
use adw::{
    ActionRow, PreferencesGroup, PreferencesPage,
    prelude::{
        ActionRowExt, AdwDialogExt, ExpanderRowExt, PreferencesDialogExt, PreferencesGroupExt,
        PreferencesPageExt,
    },
};
use gtk::glib::{DateTime, GString, Uri, UriFlags};
use indexmap::IndexMap;
use std::rc::Rc;

struct Record {
    time: DateTime,
    request: String,
    title: Option<String>,
}

pub trait Bookmarks {
    fn bookmarks(window_action: &Rc<WindowAction>, profile: &Rc<Profile>) -> Self;
}

impl Bookmarks for adw::PreferencesDialog {
    fn bookmarks(window_action: &Rc<WindowAction>, profile: &Rc<Profile>) -> Self {
        let mut index: IndexMap<GString, Vec<Record>> = IndexMap::new();
        for bookmark in profile.bookmark.recent(None) {
            match Uri::parse(&bookmark.request, UriFlags::NONE) {
                Ok(uri) => index
                    .entry(match uri.host() {
                        Some(host) => host,
                        None => uri.to_str(),
                    })
                    .or_default()
                    .push(Record {
                        request: bookmark.request,
                        time: bookmark.time,
                        title: bookmark.title,
                    }),
                Err(_) => continue, // @TODO
            }
        }

        let d = adw::PreferencesDialog::builder()
            .search_enabled(true)
            .title("Bookmarks")
            .build();

        d.add(&{
            let p = PreferencesPage::builder()
                .icon_name("document-open-recent-symbolic")
                .title("All")
                .build();

            for (group, records) in index {
                p.add(&{
                    let g = PreferencesGroup::new();
                    g.add(&{
                        let e = adw::ExpanderRow::builder()
                            .enable_expansion(true)
                            .expanded(false)
                            .subtitle(
                                records
                                    .iter()
                                    .max_by_key(|r| r.time.to_unix())
                                    .unwrap()
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
                                        None => record.time.format_iso8601().unwrap().to_string(),
                                    })
                                    .subtitle_selectable(true)
                                    .subtitle(&record.request)
                                    .build();

                                a.connect_activated({
                                    let a = window_action.clone();
                                    let d = d.clone();
                                    move |_| {
                                        a.append.activate_stateful_once(
                                            Position::After,
                                            Some(record.request.clone()),
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
