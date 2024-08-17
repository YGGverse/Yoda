#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP

#include <gtkmm/box.h>
#include <glibmm/ustring.h>

namespace app::browser::main::tab::page
{
    class Content : public Gtk::Box
    {
        public:

            Content();

            ~Content();

            void set(
                const Glib::ustring & buffer
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP