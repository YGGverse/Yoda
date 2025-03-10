mod item;

use super::Profile;
use adw::{
    prelude::{ActionRowExt, PopoverExt, PreferencesRowExt},
    ActionRow,
};
use gtk::{
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    glib::SignalHandlerId,
    prelude::{EditableExt, EntryExt, ListItemExt, WidgetExt},
    Entry, ListItem, ListView, Popover, SignalListItemFactory, SingleSelection,
};
pub use item::Item;
use std::{cell::RefCell, rc::Rc};

pub struct Suggestion {
    list_store: ListStore,
    request: Entry,
    profile: Rc<Profile>,
    popover: Popover,
    pub signal_handler_id: Rc<RefCell<Option<SignalHandlerId>>>,
}

impl Suggestion {
    // Constructors

    /// Create new `Self`
    pub fn build(profile: &Rc<Profile>, request: &Entry) -> Self {
        let list_store = ListStore::new::<Item>();
        let signal_handler_id = Rc::new(RefCell::new(None));
        Self {
            profile: profile.clone(),
            request: request.clone(),
            popover: {
                let p = Popover::builder()
                    .autohide(false)
                    .can_focus(false)
                    .halign(gtk::Align::Start)
                    .child(
                        &gtk::ScrolledWindow::builder()
                            //.css_classes(["view"])
                            .child(&{
                                let list_view = ListView::builder()
                                    .show_separators(true)
                                    .model(&{
                                        let s = SingleSelection::builder()
                                            .model(&list_store)
                                            .autoselect(false)
                                            .build();
                                        s.connect_selected_notify({
                                            let request = request.clone();
                                            let signal_handler_id = signal_handler_id.clone();
                                            move |this| {
                                                if let Some(selected_item) = this.selected_item() {
                                                    use gtk::prelude::ObjectExt;
                                                    if let Some(signal_handler_id) =
                                                        signal_handler_id.borrow().as_ref()
                                                    {
                                                        request.block_signal(signal_handler_id);
                                                    }
                                                    request.set_text(
                                                        &selected_item
                                                            .downcast_ref::<Item>()
                                                            .unwrap()
                                                            .request(),
                                                    );
                                                    request.select_region(0, -1);
                                                    if let Some(signal_handler_id) =
                                                        signal_handler_id.borrow().as_ref()
                                                    {
                                                        request.unblock_signal(signal_handler_id);
                                                    }
                                                } // @TODO find signal to handle selected item only
                                            }
                                        });
                                        s
                                    })
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
                                    .build();
                                list_view.connect_activate({
                                    let request = request.clone();
                                    move |_, _| request.emit_activate()
                                });
                                list_view
                            })
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
            signal_handler_id,
            list_store,
        }
    }

    // Actions

    pub fn update(&self, limit: Option<usize>) {
        use gtk::prelude::EditableExt;
        use itertools::Itertools;
        if self.request.text_length() > 0 {
            self.list_store.remove_all();
            let query = self.request.text();
            let items = self.profile.bookmark.contains_request(&query, limit);
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
                self.popover
                    .child()
                    .unwrap()
                    .downcast_ref::<gtk::ScrolledWindow>()
                    .unwrap();
                self.popover.popup();
                return;
            }
        }
        self.hide();
    }

    pub fn hide(&self) {
        self.popover.popdown()
    }

    pub fn to_back(&self) -> bool {
        false // @TODO
    }
    pub fn to_next(&self) -> bool {
        false // @TODO
    }

    // Getters

    pub fn is_visible(&self) -> bool {
        self.popover.is_visible()
    }

    /*pub fn total(&self) -> u32 {
        self.list_store.n_items()
    }*/
}
