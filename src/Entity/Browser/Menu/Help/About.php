<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Help;

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
    public const DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_VERSION = '<a href="https://github.com/YGGverse/Yoda"><span underline="none">Yoda</span></a> dev'; // @TODO
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_VERSION = '<a href="https://github.com/php/php-src"><span underline="none">PHP</span></a> %d.%d.%d';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_VERSION = '<a href="https://github.com/scorninpc/php-gtk3"><span underline="none">PHP-GTK</span></a> %s';
    public const DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_VERSION = 'GTK %d.%d.%d';

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
                $dialog = new GtkMessageDialog(
                    $this->help->menu->browser->gtk,
                    GtkDialogFlags::MODAL,
                    GtkMessageType::INFO,
                    GtkButtonsType::OK,
                    _($this::DIALOG_MESSAGE_FORMAT)
                );

                $dialog->format_secondary_markup(
                    implode(
                        PHP_EOL,
                        [
                            _($this::DIALOG_FORMAT_SECONDARY_MARKUP_APP_SRC_VERSION),
                            sprintf(
                                _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_SRC_VERSION),
                                PHP_MAJOR_VERSION,
                                PHP_MINOR_VERSION,
                                PHP_RELEASE_VERSION
                            ),
                            implode(
                                ' / ',
                                [
                                    sprintf(
                                        _($this::DIALOG_FORMAT_SECONDARY_MARKUP_PHP_GTK_VERSION),
                                        phpversion(
                                            $this::PHP_VERSION_GTK_EXTENSION
                                        )
                                    ),
                                    sprintf(
                                        _($this::DIALOG_FORMAT_SECONDARY_MARKUP_LIB_GTK_VERSION),
                                        0, // @TODO pending for PR #153
                                        0,
                                        0
                                    )
                                ]
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