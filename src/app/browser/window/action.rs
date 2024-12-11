mod append;
mod bookmark;
mod close;
mod close_all;
mod history_back;
mod history_forward;
mod home;
mod pin;
mod reload;
mod save_as;

use append::Append;
use bookmark::Bookmark;
use close::Close;
use close_all::CloseAll;
use history_back::HistoryBack;
use history_forward::HistoryForward;
use home::Home;
use pin::Pin;
use reload::Reload;
use save_as::SaveAs;

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
    pub append: Rc<Append>,
    pub bookmark: Rc<Bookmark>,
    pub close_all: Rc<CloseAll>,
    pub close: Rc<Close>,
    pub history_back: Rc<HistoryBack>,
    pub history_forward: Rc<HistoryForward>,
    pub home: Rc<Home>,
    pub pin: Rc<Pin>,
    pub reload: Rc<Reload>,
    pub save_as: Rc<SaveAs>,
    // Group
    pub id: GString,
    pub gobject: SimpleActionGroup,
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
        let save_as = Rc::new(SaveAs::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let gobject = SimpleActionGroup::new();

        // Add action to given group
        gobject.add_action(&append.gobject);
        gobject.add_action(&bookmark.gobject);
        gobject.add_action(&close_all.gobject);
        gobject.add_action(&close.gobject);
        gobject.add_action(&history_back.gobject);
        gobject.add_action(&history_forward.gobject);
        gobject.add_action(&home.gobject);
        gobject.add_action(&pin.gobject);
        gobject.add_action(&reload.gobject);
        gobject.add_action(&save_as.gobject);

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
            save_as,
            id,
            gobject,
        }
    }
}
