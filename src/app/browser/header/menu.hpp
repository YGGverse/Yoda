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
            Glib::RefPtr<Gio::Menu> main_tab,
            Glib::RefPtr<Gio::Menu> main_tools
        );

            // 2 level
            static Glib::RefPtr<Gio::Menu> main_tab(
                Glib::RefPtr<Gio::Menu> main_tab_page,
                Glib::RefPtr<Gio::Menu> main_tab_close
            );

                // 3 level
                static Glib::RefPtr<Gio::Menu> main_tab_page(
                    Glib::RefPtr<Gio::Menu> main_tab_page_navigation
                );

                    // 4 level
                    static Glib::RefPtr<Gio::Menu> main_tab_page_navigation(
                        Glib::RefPtr<Gio::Menu> main_tab_page_navigation_history
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