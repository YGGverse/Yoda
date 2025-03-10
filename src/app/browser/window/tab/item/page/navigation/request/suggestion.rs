mod item;

use adw::{
    prelude::{ActionRowExt, PopoverExt, PreferencesRowExt},
    ActionRow,
};
use gtk::{
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    prelude::{EntryExt, ListItemExt, WidgetExt},
    Entry, ListItem, ListView, Popover, SignalListItemFactory, SingleSelection,
};
pub use item::Item;

pub struct Suggestion {
    list_store: ListStore,
    pub popover: Popover,
}

impl Suggestion {
    // Constructors

    /// Create new `Self`
    pub fn build(request: &Entry) -> Self {
        let list_store = ListStore::new::<Item>();
        Self {
            popover: {
                let p = Popover::builder()
                    .autohide(false)
                    .can_focus(false)
                    .halign(gtk::Align::Start)
                    .child(
                        &gtk::ScrolledWindow::builder()
                            //.css_classes(["view"])
                            .child(
                                &ListView::builder()
                                    .model(
                                        &SingleSelection::builder()
                                            .model(&list_store)
                                            .autoselect(false)
                                            .build(),
                                    )
                                    .factory(&{
                                        let f = SignalListItemFactory::new();
                                        f.connect_setup(|_, this| {
                                            this.downcast_ref::<ListItem>().unwrap().set_child(
                                                Some(
                                                    &ActionRow::builder()
                                                        .use_markup(true)
                                                        .use_underline(true)
                                                        .build(),
                                                ),
                                            )
                                        });
                                        f.connect_bind(|_, this| {
                                            let l = this.downcast_ref::<ListItem>().unwrap();
                                            let i = l.item().and_downcast::<Item>().unwrap();
                                            let r = l.child().and_downcast::<ActionRow>().unwrap();
                                            r.set_title(&i.title());
                                            r.set_subtitle(&i.subtitle());
                                        });
                                        f
                                    })
                                    .build(),
                            )
                            .max_content_height(400)
                            .hscrollbar_policy(gtk::PolicyType::Never)
                            .propagate_natural_height(true)
                            .propagate_natural_width(true)
                            .build(),
                    )
                    .has_arrow(false)
                    .build();
                p.set_parent(request);
                p.set_offset(
                    request
                        .compute_point(request, &gtk::graphene::Point::zero())
                        .unwrap()
                        .x() as i32,
                    6,
                );
                p.connect_realize({
                    let request = request.clone();
                    move |this| this.set_width_request(request.width())
                });
                p
            },
            list_store,
        }
    }

    pub fn update(&self, profile: &super::Profile, request: &Entry, limit: Option<usize>) {
        use gtk::prelude::EditableExt;
        use itertools::Itertools;
        if request.text_length() > 0 {
            self.list_store.remove_all();
            let query = request.text();
            let items = profile.bookmark.contains_request(&query, limit);
            if !items.is_empty() {
                for item in items
                    .into_iter()
                    .sorted_by(|a, b| Ord::cmp(&b.request, &a.request))
                {
                    self.list_store.append(&Item::build(
                        item.request.replace(&*query, &format!("<b>{query}</b>")),
                        item.request.clone(),
                        item.request.clone(),
                    )); // @TODO
                }
                self.popover.popup();
                return;
            }
        }
        self.popover.popdown();
    }
}
