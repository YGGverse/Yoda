<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \GtkButtonsType;
use \GtkDialogFlags;
use \GtkMessageDialog;
use \GtkMessageType;
use \GtkResponseType;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

class Auth
{
    // GTK
    public GtkMessageDialog $gtk;

    // Dependencies
    public Page $page;

    // Defaults
    public const DIALOG_MESSAGE_FORMAT = 'Authorization';
    public const DIALOG_DEFAULT_RESPONSE = GtkResponseType::CANCEL;

    public function __construct(
        Page $page,
    ) {
        // Init dependencies
        $this->page = $page;
    }

    public function dialog(): bool
    {
        $this->gtk = new GtkMessageDialog(
            $this->page->container->browser->gtk,
            GtkDialogFlags::MODAL,
            GtkMessageType::INFO,
            GtkButtonsType::OK_CANCEL,
            _($this::DIALOG_MESSAGE_FORMAT)
        );

        $this->gtk->set_default_response(
            $this::DIALOG_DEFAULT_RESPONSE
        );

        if (GtkResponseType::OK == $this->gtk->run())
        {
            // @TODO
            $this->gtk->destroy();

            return true;
        }

        // Dialog canceled
        $this->gtk->destroy();

        return false;
    }
}