<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \GtkButtonsType;
use \GtkDialogFlags;
use \GtkMessageDialog;
use \GtkMessageType;
use \GtkRadioButton;
use \GtkResponseType;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

class Auth
{
    // GTK
    public GtkMessageDialog $gtk;

    // Dependencies
    public Page $page;

    // Defaults
    public const DIALOG_DEFAULT_RESPONSE = GtkResponseType::CANCEL;
    public const DIALOG_FORMAT_SECONDARY_TEXT = 'Select identity';
    public const DIALOG_MESSAGE_FORMAT = 'Authorization';
    public const DIALOG_CONTENT_OPTION_LABEL_CREATE = 'Create new for this resource';
    public const DIALOG_CONTENT_OPTION_MARGIN = 8;
    public const DIALOG_CONTENT_SPACING = 8;

    public function __construct(
        Page $page,
    ) {
        // Init dependencies
        $this->page = $page;
    }

    public function dialog(): bool
    {
        // Init dialog
        $this->gtk = new GtkMessageDialog(
            $this->page->container->browser->gtk,
            GtkDialogFlags::MODAL,
            GtkMessageType::INFO,
            GtkButtonsType::OK_CANCEL,
            _($this::DIALOG_MESSAGE_FORMAT)
        );

        $this->gtk->format_secondary_text(
            _($this::DIALOG_FORMAT_SECONDARY_TEXT)
        );

        $this->gtk->set_default_response(
            $this::DIALOG_DEFAULT_RESPONSE
        );

        // Init content
        $content = $this->gtk->get_content_area();

        $content->set_spacing(
            $this::DIALOG_CONTENT_SPACING
        );

        $content->add(
            $this->_option(
                _($this::DIALOG_CONTENT_OPTION_LABEL_CREATE)
            )
        );

        // Render
        $this->gtk->show_all();

        // Listen for chose
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

    private function _option(
        string $label,
        int $margin = self::DIALOG_CONTENT_OPTION_MARGIN
    ): GtkRadioButton
    {
        $option = GtkRadioButton::new_with_label(
            $label
        );

        $option->set_margin_start(
            $margin
        );

        $option->set_margin_start(
            $margin
        );

        $option->set_margin_end(
            $margin
        );

        $option->set_margin_bottom(
            $margin
        );

        // @TODO connect signals?

        return $option;
    }
}