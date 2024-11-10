mod append;
mod home;
mod pin;
mod reload;

use append::Append;
use home::Home;
use pin::Pin;
use reload::Reload;

use gtk::{
    gio::SimpleActionGroup,
    glib::{uuid_string_random, GString},
    prelude::ActionMapExt,
};
use std::rc::Rc;

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Browser` actions
pub struct Action {
    // Actions
    append: Rc<Append>,
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
        let home = Rc::new(Home::new());
        let pin = Rc::new(Pin::new());
        let reload = Rc::new(Reload::new());

        // Generate unique group ID
        let id = uuid_string_random();

        // Init group
        let gobject = SimpleActionGroup::new();

        // Add action to given group
        gobject.add_action(append.gobject());
        gobject.add_action(home.gobject());
        gobject.add_action(pin.gobject());
        gobject.add_action(reload.gobject());

        // Done
        Self {
            append,
            home,
            pin,
            reload,
            id,
            gobject,
        }
    }

    // Getters

    /// Get reference `Append` action
    pub fn append(&self) -> &Rc<Append> {
        &self.append
    }

    /// Get reference `Home` action
    pub fn home(&self) -> &Rc<Home> {
        &self.home
    }

    /// Get reference `Pin` action
    pub fn pin(&self) -> &Rc<Pin> {
        &self.pin
    }

    /// Get reference `Reload` action
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
