//! Browser history dialog

use super::Profile;
use crate::app::browser::window::action::{Action as WindowAction, Position};
use adw::{
    ActionRow, ExpanderRow, PreferencesDialog, PreferencesGroup, PreferencesPage,
    prelude::{
        ActionRowExt, AdwDialogExt, ExpanderRowExt, PreferencesDialogExt, PreferencesGroupExt,
        PreferencesPageExt,
    },
};
use gtk::{
    Align, Button,
    glib::{DateTime, GString, Uri, UriFlags},
    prelude::ButtonExt,
};
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

impl History for PreferencesDialog {
    fn history(window_action: &Rc<WindowAction>, profile: &Rc<Profile>) -> Self {
        let d = adw::PreferencesDialog::builder()
            .search_enabled(true)
            .title("History")
            .build();

        d.add(&page(
            window_action,
            &d,
            index(
                profile
                    .history
                    .recently_opened(None)
                    .into_iter()
                    .map(|i| (i.request, i.title, i.opened.time, i.opened.count))
                    .collect(),
            ),
            "document-open-recent-symbolic",
            "Last visit",
        ));

        d.add(&page(
            window_action,
            &d,
            index(
                profile
                    .history
                    .recently_closed(None)
                    .into_iter()
                    .map(|i| (i.request, i.title, i.opened.time, i.opened.count))
                    .collect(),
            ),
            "document-revert-symbolic",
            "Recent close",
        ));
        d
    }
}

/// Common index map for all history types
/// * @TODO make Profile member public to replace the tuple?
fn index(
    index: Vec<(GString, Option<GString>, DateTime, usize)>,
) -> IndexMap<GString, Vec<Record>> {
    let mut i: IndexMap<GString, Vec<Record>> = IndexMap::new();
    for (request, title, time, count) in index {
        match Uri::parse(&request, UriFlags::NONE) {
            Ok(uri) => i
                .entry(match uri.host() {
                    Some(host) => host,
                    None => uri.to_str(),
                })
                .or_default()
                .push(Record {
                    event: Event { time, count },
                    request,
                    title,
                }),
            Err(_) => continue, // @TODO
        }
    }
    i
}

/// Common page UI for all widget tabs
fn page(
    window_action: &Rc<WindowAction>,
    dialog: &PreferencesDialog,
    index: IndexMap<GString, Vec<Record>>,
    icon_name: &str,
    title: &str,
) -> PreferencesPage {
    let p = PreferencesPage::builder()
        .icon_name(icon_name)
        .title(title)
        .build();

    for (group, records) in index {
        p.add(&{
            let g = PreferencesGroup::new();
            g.add(&{
                let e = ExpanderRow::builder()
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
                    .title(escape(&group))
                    .build();

                for record in records {
                    e.add_row(&{
                        let a = ActionRow::builder()
                            .activatable(false)
                            .title_selectable(true)
                            .title(match record.title {
                                Some(title) => escape(&title),
                                None => format!(
                                    "{} ({})",
                                    record.event.time.format_iso8601().unwrap(),
                                    record.event.count
                                ),
                            })
                            .subtitle(escape(&record.request))
                            .subtitle_selectable(true)
                            .build();

                        a.add_prefix(
                            &Button::builder()
                                .css_classes(["circular", "caption-heading"])
                                .label(record.event.count.to_string())
                                .tooltip_text("Visit count")
                                .valign(Align::Center)
                                .build(),
                        );
                        a.add_suffix(&{
                            let b = Button::builder()
                                .css_classes(["accent", "circular", "flat"])
                                .icon_name("mail-forward-symbolic")
                                .tooltip_text("Open in the new tab")
                                .valign(Align::Center)
                                .build();
                            b.connect_clicked({
                                let a = window_action.clone();
                                let d = dialog.clone();
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
                            b
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
}

/// Prevents GTK warnings (`use_markup` has no effect @TODO)
fn escape(value: &str) -> String {
    value.replace("&amp;", "&").replace("&", "&amp;")
}
