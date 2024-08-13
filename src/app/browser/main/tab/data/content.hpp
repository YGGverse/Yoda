#ifndef APP_BROWSER_MAIN_TAB_DATA_CONTENT_HPP
#define APP_BROWSER_MAIN_TAB_DATA_CONTENT_HPP

#include <gtkmm/box.h>

namespace app::browser::main::tab::data
{
    class Content : public Gtk::Box
    {
        public:

            Content();

            ~Content();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_CONTENT_HPP