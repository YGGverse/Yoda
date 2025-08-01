mod database;
mod identity;
mod info;
mod primary_icon;
mod search;
mod suggestion;

use super::{ItemAction, Profile};
use adw::{AlertDialog, prelude::AdwDialogExt};
use anyhow::Result;
use gtk::{
    Entry, EntryIconPosition, StateFlags,
    gio::{Cancellable, ProxyResolver},
    glib::{GString, Uri, UriFlags, gformat},
    prelude::{EditableExt, EntryExt, ProxyResolverExt, WidgetExt},
};
use info::Info;
use primary_icon::PrimaryIcon;
use sqlite::Transaction;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use suggestion::Suggestion;

const PREFIX_DOWNLOAD: &str = "download:";
const PREFIX_SOURCE: &str = "source:";

pub struct Request {
    pub entry: Entry,
    pub info: Rc<RefCell<Info>>,
    profile: Rc<Profile>,
    proxy_resolver: Rc<RefCell<Option<ProxyResolver>>>,
    suggestion: Rc<Suggestion>,
}

impl Request {
    // Constructors

    /// Build new `Self`
    pub fn build(item_action: &Rc<ItemAction>, profile: &Rc<Profile>) -> Self {
        // Init components
        let info = Rc::new(RefCell::new(Info::new()));
        let proxy_resolver = Rc::new(RefCell::new(None));

        // Init main widget
        let entry = Entry::builder()
            .placeholder_text("URL or search term...")
            .hexpand(true)
            .build();

        update_primary_icon(&entry, profile);

        let suggestion = Rc::new(Suggestion::build(&entry, profile, &proxy_resolver));

        entry.add_controller({
            use gtk::{
                gdk::{Key, ModifierType},
                glib::Propagation,
            };
            let c = gtk::EventControllerKey::builder().build();
            c.connect_key_pressed({
                let entry = entry.clone();
                let suggestion = suggestion.clone();
                move |_, k, _, m| {
                    if suggestion.is_visible()
                        && !matches!(
                            m,
                            ModifierType::SHIFT_MASK
                                | ModifierType::ALT_MASK
                                | ModifierType::CONTROL_MASK
                        )
                    {
                        if matches!(k, Key::Up | Key::KP_Up | Key::Page_Up | Key::KP_Page_Up) {
                            if !suggestion.back() {
                                entry.error_bell()
                            }
                            return Propagation::Stop;
                        } else if matches!(
                            k,
                            Key::Down | Key::KP_Down | Key::Page_Down | Key::KP_Page_Down
                        ) {
                            if !suggestion.next() {
                                entry.error_bell()
                            }
                            return Propagation::Stop;
                        }
                    }
                    Propagation::Proceed
                }
            });
            c
        });

        entry.connect_icon_release({
            let i = info.clone();
            let p = profile.clone();
            move |e, position| match position {
                EntryIconPosition::Primary => {
                    if matches!(primary_icon::from(&e.text()), PrimaryIcon::Search { .. }) {
                        show_search_dialog(e, &p)
                    } else {
                        show_identity_dialog(e, &p)
                    }
                }
                EntryIconPosition::Secondary => {
                    if is_focused(e) {
                        e.emit_activate()
                    } else {
                        i.borrow().dialog(e, &p);
                    }
                }
                _ => panic!(),
            }
        });

        entry.connect_has_focus_notify({
            let i = info.clone();
            move |e| update_secondary_icon(e, &i.borrow())
        });

        suggestion
            .signal_handler_id
            .borrow_mut()
            .replace(entry.connect_changed({
                let a = item_action.clone();
                let i = info.clone();
                let p = profile.clone();
                let r = proxy_resolver.clone();
                let s = suggestion.clone();
                move |e| {
                    // Allocate once
                    let t = e.text();
                    // Update actions
                    a.reload.set_enabled(!t.is_empty());
                    a.home.set_enabled(home(e).is_some());
                    // Update icons
                    update_primary_icon(e, &p);
                    update_secondary_icon(e, &i.borrow());
                    // Show search suggestions
                    if e.focus_child().is_some() {
                        s.update(Some(50)); // @TODO optional
                    }
                    refresh_proxy_resolver(e, &p, &r)
                }
            })); // `suggestion` wants `signal_handler_id` to block this event on autocomplete navigation

        entry.connect_activate({
            let a = item_action.clone();
            let s = suggestion.clone();
            move |_| {
                use gtk::prelude::ActionExt;
                a.reload.activate(None);
                s.hide();
            }
        });

        entry.connect_has_focus_notify({
            let s = suggestion.clone();
            move |_| {
                if s.is_visible() {
                    s.hide()
                }
            }
        });

        entry.connect_state_flags_changed({
            // Define last focus state container
            let has_focus = Cell::new(false);
            move |this, state| {
                // Select entire text on first click (release)
                // this behavior implemented in most web-browsers,
                // to simply overwrite current request with new value
                // Note:
                // * Custom GestureClick is not an option here, as GTK Entry has default controller
                // * This is experimental feature does not follow native GTK behavior @TODO make optional
                if !has_focus.take()
                    && state.contains(StateFlags::ACTIVE | StateFlags::FOCUS_WITHIN)
                    && this.selection_bounds().is_none()
                {
                    this.select_region(0, -1)
                }
                // Update last focus state
                has_focus.replace(state.contains(StateFlags::FOCUS_WITHIN));
            }
        });

        Self {
            entry,
            info,
            profile: profile.clone(),
            proxy_resolver,
            suggestion,
        }
    }

    // Actions

    pub fn escape(&self) {
        self.suggestion.hide()
    }

    pub fn show_identity_dialog(&self) {
        show_identity_dialog(&self.entry, &self.profile)
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<()> {
        for record in database::select(transaction, app_browser_window_tab_item_page_navigation_id)?
        {
            database::delete(transaction, &record.id)?;
            // Delegate clean action to the item childs
            // nothing yet..
        }
        Ok(())
    }

    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<()> {
        for record in database::select(transaction, app_browser_window_tab_item_page_navigation_id)?
        {
            if let Some(text) = record.text {
                self.entry.set_text(&text);
            }
            // Delegate restore action to the item childs
            // nothing yet..
        }
        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<()> {
        // Keep value in memory until operation complete
        let text = self.entry.text();
        let _id = database::insert(
            transaction,
            app_browser_window_tab_item_page_navigation_id,
            match text.is_empty() {
                true => None,
                false => Some(text.as_str()),
            },
        )?;
        // Delegate save action to childs
        // nothing yet..
        Ok(())
    }

    pub fn refresh_proxy_resolver(&self) {
        refresh_proxy_resolver(&self.entry, &self.profile, &self.proxy_resolver)
    }

    pub fn update_secondary_icon(&self, info: &Info) {
        update_secondary_icon(&self.entry, info);
    }

    // Setters

    pub fn to_download(&self) {
        self.entry.set_text(&self.download());
    }

    pub fn to_source(&self) {
        self.entry.set_text(&self.source());
    }

    // Getters

    /// Try build home [Uri](https://docs.gtk.org/glib/struct.Uri.html) for `Self`
    /// * return `None` if current request already match home or Uri not parsable
    pub fn home(&self) -> Option<Uri> {
        home(&self.entry)
    }

    /// Try get current request value as [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    /// * `strip_prefix` on parse
    pub fn uri(&self) -> Option<Uri> {
        uri(&self.entry)
    }

    pub fn is_file(&self) -> bool {
        self.entry.text().starts_with("file://")
    }

    /// Get [ProxyResolver](https://docs.gtk.org/gio/iface.ProxyResolver.html)
    /// which is constructed for every `Request` entry change
    /// * useful on build new [SocketClient](https://docs.gtk.org/gio/class.SocketClient.html)
    ///   to prevent double search / construct operations
    pub fn proxy_resolver(&self) -> Option<ProxyResolver> {
        self.proxy_resolver.borrow().clone()
    }

    // Tools

    /// Get request value with formatted `download` prefix
    fn download(&self) -> GString {
        gformat!("{PREFIX_DOWNLOAD}{}", prefix_less(&self.entry))
    }

    /// Get request value with formatted `source` prefix
    fn source(&self) -> GString {
        gformat!("{PREFIX_SOURCE}{}", prefix_less(&self.entry))
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}

fn update_primary_icon(entry: &Entry, profile: &Profile) {
    entry.first_child().unwrap().remove_css_class("success"); // @TODO handle

    match primary_icon::from(&entry.text()) {
        PrimaryIcon::Download { name, tooltip }
        | PrimaryIcon::File { name, tooltip }
        | PrimaryIcon::Source { name, tooltip }
        | PrimaryIcon::Nex { name, tooltip } => {
            entry.set_primary_icon_activatable(false);
            entry.set_primary_icon_sensitive(false);
            entry.set_primary_icon_name(Some(name));
            entry.set_primary_icon_tooltip_text(Some(tooltip));
        }
        PrimaryIcon::Gemini { name, tooltip } | PrimaryIcon::Titan { name, tooltip } => {
            entry.set_primary_icon_activatable(true);
            entry.set_primary_icon_sensitive(true);
            entry.set_primary_icon_name(Some(name));
            if profile.identity.get(&prefix_less(entry)).is_some() {
                entry.first_child().unwrap().add_css_class("success"); // @TODO handle
                entry.set_primary_icon_tooltip_text(Some(tooltip.1));
            } else {
                entry.set_primary_icon_tooltip_text(Some(tooltip.0));
            }
        }
        PrimaryIcon::Search { name, tooltip } => {
            entry.set_primary_icon_activatable(true);
            entry.set_primary_icon_sensitive(true);
            entry.set_primary_icon_name(Some(name));
            entry.set_primary_icon_tooltip_text(Some(tooltip));
        }
    }
}

/// Secondary icon has two modes:
/// * navigate to the location button (on the entry is focused / has edit mode)
/// * page info button with dialog window activation (on the entry is inactive)
fn update_secondary_icon(entry: &Entry, info: &Info) {
    if is_focused(entry) {
        entry.set_secondary_icon_name(Some("pan-end-symbolic"));
        entry.set_secondary_icon_tooltip_text(Some("Go to the location"))
    } else {
        if info.matches(&prefix_less(entry)) {
            entry.set_secondary_icon_name(Some("help-about-symbolic"));
            entry.set_secondary_icon_tooltip_text(Some("Page info"));
        } else {
            entry.set_secondary_icon_name(None);
            entry.set_secondary_icon_tooltip_text(None);
        }
        entry.select_region(0, 0);
    }
}

/// GTK `is_focus` / `has_focus` not an option here
/// also, this method requires from the `Entry`` to be not empty
fn is_focused(entry: &Entry) -> bool {
    entry.text_length() > 0 && entry.focus_child().is_some_and(|child| child.has_focus())
}

/// Present Identity [AlertDialog](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.AlertDialog.html) for `Self`
fn show_identity_dialog(entry: &Entry, profile: &Rc<Profile>) {
    // connect identity traits
    use identity::{Common, Unsupported};
    if let Some(uri) = uri(entry) {
        if ["gemini", "titan"].contains(&uri.scheme().as_str()) {
            return AlertDialog::common(
                profile,
                &uri,
                &Rc::new({
                    let p = profile.clone();
                    let e = entry.clone();
                    move |is_reload| {
                        update_primary_icon(&e, &p);
                        if is_reload {
                            e.emit_activate();
                        }
                    }
                }),
            )
            .present(Some(entry));
        }
    }
    AlertDialog::unsupported().present(Some(entry));
}

/// Present Search providers [AlertDialog](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.AlertDialog.html) for `Self`
fn show_search_dialog(entry: &Entry, profile: &Rc<Profile>) {
    use search::Search;
    AlertDialog::search(profile).present(Some(entry))
}

/// Get current request value without system prefix
/// * the `prefix` is not `scheme`
fn prefix_less(entry: &Entry) -> GString {
    let mut request = entry.text();

    if let Some(postfix) = request.strip_prefix(PREFIX_SOURCE) {
        request = postfix.into()
    }
    if let Some(postfix) = request.strip_prefix(PREFIX_DOWNLOAD) {
        request = postfix.into()
    }
    request
}

/// Try get current request value as [Uri](https://docs.gtk.org/glib/struct.Uri.html)
/// * `strip_prefix` on parse
fn uri(entry: &Entry) -> Option<Uri> {
    Uri::parse(&prefix_less(entry), UriFlags::NONE).ok()
}

/// Try build home [Uri](https://docs.gtk.org/glib/struct.Uri.html) for `Self`
/// * return `None` if current request already match home or Uri not parsable
fn home(entry: &Entry) -> Option<Uri> {
    let uri = uri(entry)?;
    if uri.path().len() > 1 || uri.query().is_some() || uri.fragment().is_some() {
        Some(Uri::build(
            UriFlags::NONE,
            &if uri.scheme() == "titan" {
                GString::from("gemini")
            } else {
                uri.scheme()
            },
            uri.userinfo().as_deref(),
            uri.host().as_deref(),
            uri.port(),
            "/",
            None,
            None,
        ))
    } else {
        None
    }
}

/// Update request text with indicator icon without given signal emission
fn update_blocked(
    profile: &Profile,
    entry: &Entry,
    signal_handler_id: &gtk::glib::SignalHandlerId,
    text: &str,
    resolver: &RefCell<Option<ProxyResolver>>,
) {
    use gtk::prelude::ObjectExt;
    entry.block_signal(signal_handler_id);
    entry.set_text(text);
    entry.select_region(0, -1);
    update_primary_icon(entry, profile);
    refresh_proxy_resolver(entry, profile, resolver);
    entry.unblock_signal(signal_handler_id);
}

/// Indicate proxy connections @TODO cancel previous operation on update
/// * note: it's important to use sync lookup method by the current impl
fn refresh_proxy_resolver(
    entry: &Entry,
    profile: &Profile,
    resolver: &RefCell<Option<ProxyResolver>>,
) {
    const NONE: &[&str] = &[];

    let t = entry.text(); // allocate once

    match profile.proxy.matches(&t) {
        Some(m) => {
            let (css_classes, tooltip_text) = match m.lookup(&t, Cancellable::NONE) {
                Ok(h) => (&["accent"], format!("Proxy over {}", h.join(","))),
                Err(i) => (&["error"], i.to_string()),
            };
            entry.set_css_classes(if profile.proxy.misc.is_highlight_request_entry() {
                css_classes
            } else {
                NONE
            });
            entry.set_tooltip_text(Some(&tooltip_text));
            resolver.replace(Some(m));
        }
        None => {
            entry.set_css_classes(NONE);
            entry.set_tooltip_text(None);
            resolver.replace(None);
        }
    }
}
