#ifndef APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_BASE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_BASE_HPP

#include <giomm/simpleaction.h>
#include <glib.h>
#include <glibmm/i18n.h>
#include <glibmm/ustring.h>
#include <gtkmm/button.h>

namespace app::browser::main::tab::page::navigation
{
    class Base : public Gtk::Button
    {
        /*
         * Internal members
         */
        Glib::RefPtr<Gio::SimpleAction> action__open_link_variant;

        GUri * uri;

        /*
         * Base class API
         */
        public:

            Base(
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__OPEN_LINK_VARIANT
            );

            // Actions
            void update(
                const Glib::ustring & URI
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_NAVIGATION_BASE_HPP