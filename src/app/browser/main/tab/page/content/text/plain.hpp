#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_HPP

#include <glibmm/ustring.h>
#include <gtkmm/label.h>

namespace app::browser::main::tab::page::content::text
{
    class Plain : public Gtk::Label
    {
        public:

            Plain(
                const Glib::ustring & TEXT
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_TEXT_PLAIN_HPP