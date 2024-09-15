#ifndef APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP

#include <giomm/simpleaction.h>
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

            // Actions
            Glib::RefPtr<Gio::SimpleAction> action__open_link_variant;

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

            Content(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__OPEN_LINK_VARIANT
            );

            ~Content();

            // Actions
            void update(
                const MIME & MIME,
                const Glib::ustring & SOURCE,
                GUri * uri
            );

            // Getters
            Glib::ustring get_title();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_CONTENT_HPP