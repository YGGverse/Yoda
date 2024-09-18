#ifndef APP_BROWSER_MAIN_TAB_PAGE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_HPP

#include <giomm/asyncresult.h>
#include <giomm/inputstream.h>
#include <giomm/outputstream.h>
#include <giomm/simpleaction.h>
#include <giomm/simpleactiongroup.h>
#include <giomm/socket.h>
#include <giomm/socketclient.h>
#include <giomm/socketconnection.h>
#include <glibmm/i18n.h>
#include <glibmm/main.h>
#include <glibmm/refptr.h>
#include <glibmm/regex.h>
#include <glibmm/stringutils.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>
#include <gtkmm/object.h>
#include <sqlite3.h>

namespace app::browser::main::tab
{
    namespace page
    {
        class Content;
        class Navigation;
    }

    class Page : public Gtk::Box
    {
        public:

            enum class MIME
            {
                TEXT_PLAIN,
                TEXT_GEMINI,
                UNDEFINED
            };

            /*
             * Page class database
             *
             * Allowed parental access to enums and relationship methods
             */
            struct Database
            {
                // app_browser_main_tab_page__*
                struct Session
                {
                    enum
                    {
                        ID,
                        APP_BROWSER_MAIN_TAB__SESSION__ID,
                        TIME,
                        MIME,
                        TITLE,
                        DESCRIPTION
                    }; // table fields index

                    static int init(
                        sqlite3 * database
                    ); // return sqlite3_exec status code

                    static int clean(
                        sqlite3 * database,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
                    ); // return sqlite3_finalize status code

                    static sqlite3_int64 add(
                        sqlite3 * database,
                        const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID,
                        const Page::MIME & MIME,
                        const Glib::ustring & TITLE,
                        const Glib::ustring & DESCRIPTION
                    ); // return sqlite3_last_insert_rowid
                };
            };

            /*
             * Socket helpers
             *
             * Page class works with multiple protocols and requires some extended features below
             */
            struct Socket
            {
                struct Buffer
                {
                    // Defaults
                    static const size_t DEFAULT_SIZE = 0xfffff; // 1Mb

                    // Stream
                    char data[DEFAULT_SIZE];

                    // Tools
                    unsigned long capacity();
                    void clean();
                };

                class Client
                {
                    private:

                        static Glib::RefPtr<Gio::SocketClient> create(
                            const int & TIMEOUT = 15
                        );

                    public:

                        /*
                         * Gemini protocol
                         *
                         * https://geminiprotocol.net
                         */
                        struct Gemini
                        {
                            // Defaults
                            static const int DEFAULT_PORT = 1965;

                            // Actions
                            static Glib::RefPtr<Gio::SocketClient> create();

                            struct Request
                            {
                                static Glib::ustring create_from_uri(
                                    GUri * uri
                                );
                            };

                            struct Response
                            {
                                /*
                                 * Status codes
                                 *
                                 * 10-19 Input expected
                                 * 20-29 Success
                                 * 30-39 Redirection
                                 * 40-49 Temporary failure
                                 * 50-59 Permanent failure
                                 * 60-69 Client certificates
                                 *
                                 * https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
                                 */
                                enum class Status
                                {
                                    INPUT,
                                    SUCCESS,
                                    REDIRECT,
                                    TEMPORARY_FAILURE,
                                    PERMANENT_FAILURE,
                                    CERTIFICATE,
                                    UNDEFINED
                                }; // @TODO explain code groups

                                struct Match
                                {
                                    static bool meta(
                                        const Glib::ustring & RESPONSE,
                                        Status & status,
                                        MIME & mime // same to page global
                                    );
                                };
                            };
                        };
                };

                struct Connection
                {
                    static bool is_active(
                        const Glib::RefPtr<Gio::SocketConnection> & CONNECTION
                    );

                    static bool close(
                        Glib::RefPtr<Gio::SocketConnection> & connection
                    );
                };
            };

        /*
         * Internal members
         */
        private:

            // Meta
            MIME mime;

            double progress_fraction;

            GUri * uri;

            Glib::ustring title;
            Glib::ustring description;

            // Actions
            Glib::RefPtr<Gio::SimpleAction> action__update;

            // Database
            sqlite3 * database;

            // Socket
            Socket::Buffer buffer;

            // Shared socket connectors (for async operations)
            Glib::RefPtr<Gio::SocketClient> socket__client;
            Glib::RefPtr<Gio::SocketConnection> socket__connection;

            // Components
            page::Content * pageContent;
            page::Navigation * pageNavigation;

        /*
         * Class API
         */
        public:

            Page(
                sqlite3 * database,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_BACK,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__HISTORY_FORWARD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__RELOAD,
                const Glib::RefPtr<Gio::SimpleAction> & ACTION__UPDATE
            );

            // Actions
            int session_restore(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            ); // return sqlite3_finalize status code

            void session_save(
                const sqlite3_int64 & APP_BROWSER_MAIN_TAB__SESSION__ID
            );

            void update();

            void navigation_reload(
                const bool & ADD_HISTORY
            );

            void navigation_history_back();
            void navigation_history_forward();

            // Getters
            MIME get_mime();
            Glib::ustring get_title();
            Glib::ustring get_description();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP