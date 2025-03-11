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
    INVALID_LIST_POSITION,
};
pub use item::Item;
use sourceview::prelude::ListModelExt;
use std::{cell::RefCell, rc::Rc};

pub struct Suggestion {
    list_store: ListStore,
    list_view: ListView,
    single_selection: SingleSelection,
    request: Entry,
    profile: Rc<Profile>,
    popover: Popover,
    pub signal_handler_id: Rc<RefCell<Option<SignalHandlerId>>>,
}

impl Suggestion {
    // Constructors

    /// Create new `Self`
    pub fn build(profile: &Rc<Profile>, request: &Entry) -> Self {
        let signal_handler_id = Rc::new(RefCell::new(None));
        let list_store = ListStore::new::<Item>();
        let single_selection = {
            let ss = SingleSelection::builder()
                .model(&list_store)
                .autoselect(false)
                .build();
            ss.connect_selected_notify({
                let request = request.clone();
                let signal_handler_id = signal_handler_id.clone();
                move |this| {
                    if let Some(selected_item) = this.selected_item() {
                        use gtk::prelude::ObjectExt;
                        if let Some(signal_handler_id) = signal_handler_id.borrow().as_ref() {
                            request.block_signal(signal_handler_id);
                        }
                        request.set_text(&selected_item.downcast_ref::<Item>().unwrap().request());
                        request.select_region(0, -1);
                        if let Some(signal_handler_id) = signal_handler_id.borrow().as_ref() {
                            request.unblock_signal(signal_handler_id);
                        }
                    } // @TODO find signal to handle selected item only
                }
            });
            ss
        };
        let list_view = {
            let lv = ListView::builder()
                .show_separators(true)
                .model(&single_selection)
                .factory(&{
                    let f = SignalListItemFactory::new();
                    f.connect_setup(|_, this| {
                        this.downcast_ref::<ListItem>().unwrap().set_child(Some(
                            &ActionRow::builder()
                                .use_markup(true)
                                .use_underline(true)
                                .build(),
                        ))
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
            lv.connect_activate({
                let request = request.clone();
                move |_, _| request.emit_activate()
            });
            lv
        };
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
                            .child(&list_view)
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
            single_selection,
            list_store,
            list_view,
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
                self.popover.popup();
                return;
            }
        }
        self.hide();
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
