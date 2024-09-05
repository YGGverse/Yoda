#ifndef APP_BROWSER_HEADER_MENU_HPP
#define APP_BROWSER_HEADER_MENU_HPP

#include <giomm/menu.h>
#include <glibmm/i18n.h>
#include <gtkmm/menubutton.h>

namespace app::browser::header
{
    class Menu : public Gtk::MenuButton
    {
        // 1 level
        static Glib::RefPtr<Gio::Menu> main(
            const Glib::RefPtr<Gio::Menu> & MAIN_TAB,
            const Glib::RefPtr<Gio::Menu> & MAIN_TOOLS
        );

            // 2 level
            static Glib::RefPtr<Gio::Menu> main_tab(
                const Glib::RefPtr<Gio::Menu> & MAIN_TAB_PAGE,
                const Glib::RefPtr<Gio::Menu> & MAIN_TAB_CLOSE
            );

                // 3 level
                static Glib::RefPtr<Gio::Menu> main_tab_page(
                    const Glib::RefPtr<Gio::Menu> & MAIN_TAB_PAGE_NAVIGATION
                );

                    // 4 level
                    static Glib::RefPtr<Gio::Menu> main_tab_page_navigation(
                        const Glib::RefPtr<Gio::Menu> & MAIN_TAB_PAGE_NAVIGATION_HISTORY
                    );

                        // 5 level
                        static Glib::RefPtr<Gio::Menu> main_tab_page_navigation_history();

                static Glib::RefPtr<Gio::Menu> main_tab_close();

            static Glib::RefPtr<Gio::Menu> main_tools();

        public:

            Menu();
    };
}

#endif // APP_BROWSER_HEADER_MENU_HPP