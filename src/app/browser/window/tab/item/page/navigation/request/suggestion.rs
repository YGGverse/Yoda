mod item;

use super::Profile;
use adw::{
    ActionRow,
    prelude::{ActionRowExt, PopoverExt, PreferencesRowExt},
};
use gtk::{
    Align, Entry, INVALID_LIST_POSITION, ListItem, ListView, Popover, SignalListItemFactory,
    SingleSelection,
    gio::{
        ListStore,
        prelude::{Cast, CastNone},
    },
    glib::{GString, SignalHandlerId},
    prelude::{EntryExt, ListItemExt, WidgetExt},
};
pub use item::Item;
use sourceview::prelude::ListModelExt;
use std::{cell::RefCell, rc::Rc};

pub struct Suggestion {
    list_store: ListStore,
    list_view: ListView,
    single_selection: SingleSelection,
    entry: Entry,
    profile: Rc<Profile>,
    popover: Popover,
    pub signal_handler_id: Rc<RefCell<Option<SignalHandlerId>>>,
}

impl Suggestion {
    // Constructors

    /// Create new `Self`
    pub fn build(
        entry: &Entry,
        profile: &Rc<Profile>,
        resolver: &Rc<RefCell<Option<gtk::gio::ProxyResolver>>>,
    ) -> Self {
        let signal_handler_id = Rc::new(RefCell::new(None));
        let list_store = ListStore::new::<Item>();
        let single_selection = {
            let ss = SingleSelection::builder()
                .model(&list_store)
                .autoselect(false)
                .build();
            ss.connect_selected_notify({
                let e = entry.clone();
                let p = profile.clone();
                let r = resolver.clone();
                let signal_handler_id = signal_handler_id.clone();
                move |this| {
                    if let Some(selected_item) = this.selected_item() {
                        if let Some(signal_handler_id) = signal_handler_id.borrow().as_ref() {
                            super::update_blocked(
                                &p,
                                &e,
                                signal_handler_id,
                                &selected_item.downcast_ref::<Item>().unwrap().request(),
                                &r,
                            );
                        }
                    } // @TODO find signal to handle selected item only
                }
            });
            ss
        };
        let list_view = {
            let lv = ListView::builder()
                .name(format!("s{}", gtk::glib::uuid_string_random()))
                .valign(Align::Start)
                .model(&single_selection)
                .factory(&{
                    let f = SignalListItemFactory::new();
                    f.connect_setup(|_, this| {
                        let r = ActionRow::builder()
                            .use_markup(true)
                            .use_underline(true)
                            .build();
                        r.add_suffix(
                            &gtk::Image::builder()
                                .icon_name("starred-symbolic")
                                .margin_end(4)
                                .pixel_size(11)
                                .visible(false)
                                .build(),
                        );
                        this.downcast_ref::<ListItem>().unwrap().set_child(Some(&r))
                    });
                    f.connect_bind(|_, this| {
                        use gtk::prelude::ListBoxRowExt;
                        let l = this.downcast_ref::<ListItem>().unwrap();
                        let i = l.item().and_downcast::<Item>().unwrap();
                        let r = l.child().and_downcast::<ActionRow>().unwrap();
                        r.set_title(&i.title());
                        r.set_subtitle(&i.subtitle());
                        r.child()
                            .unwrap()
                            .last_child()
                            .unwrap()
                            .last_child()
                            .unwrap()
                            .set_visible(i.has_bookmark());
                    });
                    f
                })
                .build();
            lv.add_controller({
                let c = gtk::GestureClick::builder()
                    .button(gtk::gdk::BUTTON_PRIMARY)
                    .build();
                c.connect_released({
                    let e = entry.clone();
                    move |_, _, _, _| e.emit_activate()
                });
                c
            });
            lv.connect_activate({
                let e = entry.clone();
                move |_, _| e.emit_activate()
            });
            list_view_css_patch(&lv);
            lv
        };
        Self {
            profile: profile.clone(),
            entry: entry.clone(),
            popover: {
                let p = Popover::builder()
                    .autohide(false)
                    .can_focus(false)
                    .css_classes(["menu"])
                    .halign(Align::Start)
                    .valign(Align::Start)
                    .child(
                        &gtk::ScrolledWindow::builder()
                            .child(&list_view)
                            .max_content_height(400)
                            .hscrollbar_policy(gtk::PolicyType::Never)
                            .propagate_natural_height(true)
                            .propagate_natural_width(true)
                            .build(),
                    )
                    .has_arrow(false)
                    .build();
                p.set_parent(entry);
                p.set_offset(
                    entry
                        .compute_point(entry, &gtk::graphene::Point::zero())
                        .unwrap()
                        .x() as i32,
                    6,
                );
                p.connect_realize({
                    let e = entry.clone();
                    move |this| this.set_width_request(e.width() + 18)
                });
                p
            },
            signal_handler_id,
            single_selection,
            list_store,
            list_view,
        }
    }

    // Actions

    pub fn update(&self, limit: Option<usize>) {
        use gtk::prelude::EditableExt;
        use itertools::Itertools;
        if self.entry.text_length() > 0 {
            let query = self.entry.text();
            let popover = self.popover.clone();
            let list_store = self.list_store.clone();
            let profile = self.profile.clone();
            gtk::glib::spawn_future_local(async move {
                let list_items: Vec<(GString, GString, bool, GString)> = profile
                    .history
                    .contains_request(&query, limit)
                    .into_iter()
                    .sorted_by(|a, b| Ord::cmp(&b.opened.count, &a.opened.count))
                    .map(|item| {
                        let subtitle = highlight(&item.request, &query);
                        let title = match item.title {
                            Some(title) => highlight(&title, &query),
                            None => subtitle.clone(),
                        };
                        (
                            title,
                            subtitle,
                            profile.bookmark.is_match_request(&item.request),
                            item.request,
                        )
                    })
                    .sorted_by(|a, b| Ord::cmp(&b.2, &a.2)) // bookmark first
                    .collect();
                if list_items.is_empty() {
                    popover.popdown();
                } else {
                    list_store.remove_all();
                    for (title, subtitle, has_bookmark, request) in list_items {
                        list_store.append(&Item::build(title, subtitle, has_bookmark, request));
                    } // @TODO take a while
                    popover.popup()
                }
            });
        } else {
            self.popover.popdown();
        }
    }

    pub fn hide(&self) {
        self.popover.popdown()
    }

    pub fn back(&self) -> bool {
        let position = self.single_selection.selected();
        if position == 0 {
            return false; // prevent unsigned value decrement
        }
        self.select(position - 1)
    }

    pub fn next(&self) -> bool {
        let position = self.single_selection.selected();
        if position == INVALID_LIST_POSITION {
            return self.select(0);
        }
        self.select(position + 1)
    }

    // Getters

    pub fn is_visible(&self) -> bool {
        self.popover.is_visible()
    }

    // Tools

    fn select(&self, position: u32) -> bool {
        let total = self.list_store.n_items();
        if position == INVALID_LIST_POSITION || position >= total || total == 0 {
            return false;
        }
        self.single_selection.set_selected(position);
        self.list_view
            .scroll_to(position, gtk::ListScrollFlags::NONE, None);
        true
    }
}

// Tools

fn highlight(subject: &str, key: &str) -> GString {
    subject.replace(key, &format!("<b>{key}</b>")).into()
}

fn list_view_css_patch(list_view: &ListView) {
    use gtk::prelude::WidgetExt;

    let name = list_view.widget_name();
    let provider = gtk::CssProvider::new();

    provider.load_from_string(&format!(
        "
            #{name} > row {{
                padding: 0;
            }}
            #{name} > row:hover {{
                background-color: alpha(currentColor, .04);
            }}
            #{name} > row:active {{
                background-color: alpha(currentColor, .08);
            }}
            #{name} > row:selected {{
                background-color: alpha(currentColor, .12);
            }}
            #{name} > row:selected:hover {{
                background-color: alpha(currentColor, .16);
            }}
            #{name} > row:selected:active {{
                background-color: alpha(currentColor, .2);
            }}
        "
    ));

    gtk::style_context_add_provider_for_display(
        &list_view.display(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
