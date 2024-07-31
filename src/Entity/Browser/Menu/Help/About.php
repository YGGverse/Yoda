<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Help;

use \Composer\InstalledVersions;

use \Exception;

use \Gtk;
use \GtkButtonsType;
use \GtkDialogFlags;
use \GtkJustification;
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

    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_NAME = '<a href="https://github.com/YGGverse/Yoda"><span underline="none">Yoda</span></a>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_INFO = 'Browser for Gemini protocol';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_META = '<span size="small">%s</span>';

    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_NAME = '<a href="https://github.com/php/php-src"><span underline="none">PHP</span></a>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_INFO = 'Hypertext Preprocessor';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_META = '<span size="small">version: %d.%d.%d</span>';

    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_NAME = '<a href="https://github.com/scorninpc/php-gtk3"><span underline="none">PHP-GTK</span></a>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_INFO = 'Bind of GTK 3 to create desktop applications with PHP';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_META = '<span size="small">version: %s</span>';

    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_NAME = '<a href="https://gtk.org"><span underline="none">GTK</span></a>';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_INFO = 'Free and open-source cross-platform widget toolkit';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_META = '<span size="small">version: %d.%d.%d</span>';

    public const PHP_VERSION_GTK_EXTENSION = 'php-gtk3';

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
                        '%s %s',
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

                // Build dialog template
                $dialog->format_secondary_markup(
                    implode(
                        PHP_EOL,
                        [
                            // App
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_NAME),
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_INFO),
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_META),
                                implode(
                                    ' / ',
                                    $composer
                                )
                            ),
                            null,
                            // PHP
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_NAME),
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_INFO),
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_META),
                                PHP_MAJOR_VERSION,
                                PHP_MINOR_VERSION,
                                PHP_RELEASE_VERSION
                            ),
                            null,
                            // PHP-GTK
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_NAME),
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_INFO),
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_META),
                                phpversion(
                                    $this::PHP_VERSION_GTK_EXTENSION
                                )
                            ),
                            null,
                            // GTK
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_NAME),
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
                    if (!isset($label[1]))
                    {
                        throw new Exception;
                    }

                    $label[1]->set_selectable(
                        true
                    );

                    $label[1]->set_track_visited_links(
                        false
                    );

                    $label[1]->set_justify(
                        GtkJustification::CENTER
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