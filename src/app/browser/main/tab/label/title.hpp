#ifndef APP_BROWSER_MAIN_TAB_LABEL_NAME_HPP
#define APP_BROWSER_MAIN_TAB_LABEL_NAME_HPP

#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/label.h>
#include <pangomm/layout.h>

namespace app::browser::main::tab::label
{
    class Title : public Gtk::Label
    {
        /*
         * Internal members
         */
        private:

            // Defaults
            static const int WIDTH_CHARS = 16;

        /*
         * Class API
         */
        public:

            Title();
    };
}

#endif // APP_BROWSER_MAIN_TAB_LABEL_NAME_HPP