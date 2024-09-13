#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_HPP

#include <glibmm/ustring.h>
#include <gtkmm/enums.h>
#include <gtkmm/label.h>
#include <gtkmm/viewport.h>

namespace app::browser::main::tab::page::content::text
{
    class Plain : public Gtk::Viewport
    {
        public:

            Plain(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_HPP