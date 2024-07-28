<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Menu\Tab;

use \GtkButtonsType;
use \GtkDialogFlags;
use \GtkMessageDialog;
use \GtkMessageType;
use \GtkResponseType;

use \Yggverse\Yoda\Entity\Browser\Menu\Tab;

class Clean
{
    public \GtkMenuItem $gtk;

    // Dependencies
    public Tab $tab;

    // Defaults
    public const LABEL = 'Clean session';
    public const TOOLTIP = 'Close all tabs';
    public const DIALOG_MESSAGE_FORMAT = 'Clean session';
    public const DIALOG_FORMAT_SECONDARY_TEXT = 'Close all tabs and start new session?';

    public function __construct(
        Tab $tab
    ) {
        // Init dependencies
        $this->tab = $tab;

        // Init menu item
        $this->gtk = \GtkMenuItem::new_with_label(
            _($this::LABEL)
        );

        $this->gtk->set_tooltip_text(
            _($this::TOOLTIP)
        );

        // Render
        $this->gtk->show();

        // Int events
        $this->gtk->connect(
            'activate',
            function()
            {
                $dialog = new GtkMessageDialog(
                    $this->tab->menu->browser->gtk,
                    GtkDialogFlags::MODAL,
                    GtkMessageType::WARNING,
                    GtkButtonsType::OK_CANCEL,
                    $this::DIALOG_MESSAGE_FORMAT
                );

                $dialog->format_secondary_text(
                    $this::DIALOG_FORMAT_SECONDARY_TEXT
                );

                if (GtkResponseType::OK == $dialog->run())
                {
                    foreach ($this->tab->menu->browser->container->tab->pages as $page_num => $page)
                    {
                        $this->tab->menu->browser->container->tab->close(
                            $page_num
                        );
                    }
                }

                $dialog->destroy();
            }
        );
    }
}