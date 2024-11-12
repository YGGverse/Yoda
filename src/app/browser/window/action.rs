mod append;
mod bookmark;
mod close;
mod close_all;
mod history_back;
mod history_forward;
mod home;
mod pin;
mod reload;

use append::Append;
use bookmark::Bookmark;
use close::Close;
use close_all::CloseAll;
use history_back::HistoryBack;
use history_forward::HistoryForward;
use home::Home;
use pin::Pin;
use reload::Reload;

use gtk::{
    gio::SimpleActionGroup,
    glib::{uuid_string_random, GString},
    prelude::ActionMapExt,
};
use std::rc::Rc;

pub use append::Position; // public enum

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    // Actions
    append: Rc<Append>,
    bookmark: Rc<Bookmark>,
    close_all: Rc<CloseAll>,
    close: Rc<Close>,
    history_back: Rc<HistoryBack>,
    history_forward: Rc<HistoryForward>,
    home: Rc<Home>,
    pin: Rc<Pin>,
    reload: Rc<Reload>,
    // Group
    id: GString,
    gobject: SimpleActionGroup,
}

impl Action {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init actions
        let append = Rc::new(Append::new());
        let bookmark = Rc::new(Bookmark::new());
        let close = Rc::new(Close::new());
        let close_all = Rc::new(CloseAll::new());
        let history_back = Rc::new(HistoryBack::new());
        let history_forward = Rc::new(HistoryForward::new());
        let home = Rc::new(Home::new());
        let pin = Rc::new(Pin::new());
        let reload = Rc::new(Reload::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let gobject = SimpleActionGroup::new();

        // Add action to given group
        gobject.add_action(append.gobject());
        gobject.add_action(bookmark.gobject());
        gobject.add_action(close_all.gobject());
        gobject.add_action(close.gobject());
        gobject.add_action(history_back.gobject());
        gobject.add_action(history_forward.gobject());
        gobject.add_action(home.gobject());
        gobject.add_action(pin.gobject());
        gobject.add_action(reload.gobject());

        // Done
        Self {
            append,
            bookmark,
            close_all,
            close,
            history_back,
            history_forward,
            home,
            pin,
            reload,
            id,
            gobject,
        }
    }

    // Getters

    /// Get reference to `Append` action
    pub fn append(&self) -> &Rc<Append> {
        &self.append
    }

    /// Get reference to `Bookmark` action
    pub fn bookmark(&self) -> &Rc<Bookmark> {
        &self.bookmark
    }

    /// Get reference to `CloseAll` action
    pub fn close_all(&self) -> &Rc<CloseAll> {
        &self.close_all
    }

    /// Get reference to `Close` action
    pub fn close(&self) -> &Rc<Close> {
        &self.close
    }

    /// Get reference to `HistoryBack` action
    pub fn history_back(&self) -> &Rc<HistoryBack> {
        &self.history_back
    }

    /// Get reference to `HistoryForward` action
    pub fn history_forward(&self) -> &Rc<HistoryForward> {
        &self.history_forward
    }

    /// Get reference to `Home` action
    pub fn home(&self) -> &Rc<Home> {
        &self.home
    }

    /// Get reference to `Pin` action
    pub fn pin(&self) -> &Rc<Pin> {
        &self.pin
    }

    /// Get reference to `Reload` action
    pub fn reload(&self) -> &Rc<Reload> {
        &self.reload
    }

    /// Get auto-generated name for action group
    /// * useful for manual relationship with GObjects or as the `detailed_name`
    ///   for [Accels](https://docs.gtk.org/gtk4/method.Application.set_accels_for_action.html) or
    ///   [Menu](https://docs.gtk.org/gio/class.Menu.html) builder
    pub fn id(&self) -> &GString {
        &self.id
    }

    /// Get reference to [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) GObject
    pub fn gobject(&self) -> &SimpleActionGroup {
        &self.gobject
    }
}
