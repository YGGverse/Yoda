<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Help;

use \Composer\InstalledVersions;

use \Exception;

use \Gtk;
use \GtkButtonsType;
use \GtkDialogFlags;
use \GtkMenuItem;
use \GtkMessageDialog;
use \GtkMessageType;
use \GtkResponseType;

use \Yggverse\Yoda\Entity\Browser\Menu\Help;

class About
{
    // GTK
    public GtkMenuItem $gtk;

    // Dependencies
    public Help $help;

    // Defaults
    public const LABEL = 'About';

    public const DIALOG_MESSAGE_FORMAT = 'About';

    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_NAME = '<a href="%s" title="%s"><span underline="none">⇗ Yoda</span></a>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_INFO = 'Browser for Gemini protocol';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_META = '<span size="small">%s</span>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_HREF = 'https://github.com/YGGverse/Yoda';

    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_NAME = '<a href="%s" title="%s"><span underline="none">⇗ PHP</span></a>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_INFO = 'The Hypertext Preprocessor';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_META = '<span size="small">%s</span>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_HREF = 'https://php.net';

    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_NAME = '<a href="%s" title="%s"><span underline="none">⇗ GTK</span></a>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_INFO = 'Free and open-source cross-platform widget toolkit';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_META = '<span size="small">version %d.%d.%d</span>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_HREF = 'https://gtk.org';

    public function __construct(
        Help $help
    ) {
        // Init dependencies
        $this->help = $help;

        // Init menu item
        $this->gtk = GtkMenuItem::new_with_label(
            $this::LABEL
        );

        // Render
        $this->gtk->show();

        // Int events
        $this->gtk->connect(
            'activate',
            function()
            {
                // Init application info dialog
                $dialog = new GtkMessageDialog(
                    $this->help->menu->browser->gtk,
                    GtkDialogFlags::MODAL,
                    GtkMessageType::INFO,
                    GtkButtonsType::OK,
                    _($this::DIALOG_MESSAGE_FORMAT)
                );

                // Get composer versions installed
                $composer = [];

                foreach (InstalledVersions::getInstalledPackages() as $package)
                {
                    $composer[] = sprintf(
                        '%s-%s',
                        basename(
                            $package
                        ),
                        strval(
                            InstalledVersions::getVersion(
                                $package
                            )
                        )
                    );
                }

                sort(
                    $composer
                );

                // Get phpinfo
                $phpinfo = [];

                foreach (get_loaded_extensions() as $extension)
                {
                    $phpinfo[] = sprintf(
                        '%s-%s',
                        strtolower(
                            $extension
                        ),
                        strval(
                            phpversion(
                                $extension
                            )
                        )
                    );
                }

                sort(
                    $phpinfo
                );

                // Build dialog template
                $dialog->format_secondary_markup(
                    implode(
                        PHP_EOL,
                        [
                            // App
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_NAME),
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_HREF),
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_HREF)
                            ),
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_INFO),
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_META),
                                implode(
                                    ' ',
                                    $composer
                                )
                            ),
                            null,
                            // PHP
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_NAME),
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_HREF),
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_HREF)
                            ),
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_INFO),
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_META),
                                implode(
                                    ' ',
                                    $phpinfo
                                )
                            ),
                            null,
                            // GTK
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_NAME),
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_HREF),
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_HREF)
                            ),
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_INFO),
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_META),
                                Gtk::MAJOR_VERSION,
                                Gtk::MICRO_VERSION,
                                Gtk::MINOR_VERSION
                            )
                        ]
                    )
                );

                // Tune up the label
                if ($label = $dialog->get_message_area()->get_children())
                {
                    if (empty($label[0]))
                    {
                        throw new Exception;
                    }

                    $label[0]->grab_focus();

                    if (empty($label[1]))
                    {
                        throw new Exception;
                    }

                    $label[1]->set_selectable(
                        true
                    );

                    $label[1]->set_track_visited_links(
                        false
                    );
                }

                // Await for action
                if (GtkResponseType::OK == $dialog->run())
                {
                    $dialog->destroy();
                }
            }
        );
    }
}