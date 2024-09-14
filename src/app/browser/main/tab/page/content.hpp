#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP

#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/enums.h>

namespace app::browser::main::tab::page
{
    namespace content
    {
        class Text;
    }

    class Content : public Gtk::Box
    {
        /*
         * Internal members
         */
        private:

            // Components
            content::Text * contentText;

            // Extra features
            Glib::ustring title;

        /*
         * Content class API
         */
        public:

            enum MIME
            {
                TEXT_GEMINI,
                TEXT_PLAIN
            };

            Content();
            ~Content();

            // Actions
            void update(
                const MIME & MIME,
                const Glib::ustring & DATA
            );

            // Getters
            Glib::ustring get_title();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP