#pragma once

#ifndef APP_BROWSER_HEADER_MENU_MAIN_TAB_H
#define APP_BROWSER_HEADER_MENU_MAIN_TAB_H

// Dependencies
#include "../main.h"

// Requirements
#include "tab/append.h"

namespace app::browser::header::menu
{
    class Main;

    namespace main
    {
        namespace tab
        {
            class Append;
        }

        class Tab
        {
            public:

                // GTK
                GtkWidget *gtk;

                // Gio
                GMenu* model;

                GMenuItem *item;

                // Dependencies
                Main *main;

                // Requirements
                tab::Append *append;

                // Defaults
                const gchar *LABEL = "Tab";

                // Construct
                Tab(
                    Main *main
                );
        };
    };
};

#endif