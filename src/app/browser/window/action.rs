mod append;
mod bookmark;
mod close;
mod close_all;
mod find;
mod history_back;
mod history_forward;
mod home;
mod pin;
mod reload;
mod save_as;
mod source;

use append::Append;
use bookmark::Bookmark;
use close::Close;
use close_all::CloseAll;
use find::Find;
use history_back::HistoryBack;
use history_forward::HistoryForward;
use home::Home;
use pin::Pin;
use reload::Reload;
use save_as::SaveAs;
use source::Source;

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
    pub find: Rc<Find>,
    pub history_back: Rc<HistoryBack>,
    pub history_forward: Rc<HistoryForward>,
    pub home: Rc<Home>,
    pub pin: Rc<Pin>,
    pub reload: Rc<Reload>,
    pub save_as: Rc<SaveAs>,
    pub source: Rc<Source>,
    // Group
    pub id: GString,
    pub simple_action_group: SimpleActionGroup,
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
        let find = Rc::new(Find::new());
        let history_back = Rc::new(HistoryBack::new());
        let history_forward = Rc::new(HistoryForward::new());
        let home = Rc::new(Home::new());
        let pin = Rc::new(Pin::new());
        let reload = Rc::new(Reload::new());
        let save_as = Rc::new(SaveAs::new());
        let source = Rc::new(Source::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let simple_action_group = SimpleActionGroup::new();

        // Add action to given group
        simple_action_group.add_action(&append.simple_action);
        simple_action_group.add_action(&bookmark.simple_action);
        simple_action_group.add_action(&close_all.simple_action);
        simple_action_group.add_action(&close.simple_action);
        simple_action_group.add_action(&find.simple_action);
        simple_action_group.add_action(&history_back.simple_action);
        simple_action_group.add_action(&history_forward.simple_action);
        simple_action_group.add_action(&home.simple_action);
        simple_action_group.add_action(&pin.simple_action);
        simple_action_group.add_action(&reload.simple_action);
        simple_action_group.add_action(&save_as.simple_action);
        simple_action_group.add_action(&source.simple_action);

        // Done
        Self {
            append,
            bookmark,
            close_all,
            close,
            find,
            history_back,
            history_forward,
            home,
            pin,
            reload,
            save_as,
            source,
            id,
            simple_action_group,
        }
    }
}
