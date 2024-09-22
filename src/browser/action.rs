use gtk::gio::ActionEntry;
use gtk::prelude::GtkWindowExt;
use gtk::ApplicationWindow;

pub fn debug() -> ActionEntry<ApplicationWindow> {
    ActionEntry::builder("debug")
        .activate(|this: &ApplicationWindow, _, _| {
            this.emit_enable_debugging(true);
        })
        .build()
}

pub fn quit() -> ActionEntry<ApplicationWindow> {
    ActionEntry::builder("quit")
        .activate(|this: &ApplicationWindow, _, _| {
            this.close();
        })
        .build()
}

/* @TODO
pub fn tab_append(main) -> ActionEntry<ApplicationWindow> {
    let action_tab_append = ActionEntry::builder("tab_append")
        .activate({
            let main = main.clone();
            move |_, _, _| {
                main.tab_append();
            }
        })
        .build();
}*/
