#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_BOOKMARK_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_BOOKMARK_HPP

#include <glibmm/i18n.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navbar
{
    class Bookmark : public Gtk::Button
    {
        public:

            Bookmark();

            ~Bookmark();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVBAR_BOOKMARK_HPP