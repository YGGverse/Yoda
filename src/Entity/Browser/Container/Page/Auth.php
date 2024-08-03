<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Exception;
use \GtkButtonsType;
use \GtkDialogFlags;
use \GtkEntry;
use \GtkMessageDialog;
use \GtkMessageType;
use \GtkRadioButton;
use \GtkResponseType;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Model\Identity\Gemini;

class Auth
{
    // GTK
    public GtkMessageDialog $gtk;

    // Dependencies
    public Page $page;

    // Requirements
    public GtkEntry $name;

    // Defaults
    public const DIALOG_DEFAULT_RESPONSE = GtkResponseType::CANCEL;
    public const DIALOG_FORMAT_SECONDARY_TEXT = 'Select identity';
    public const DIALOG_MESSAGE_FORMAT = 'Authorization';
    public const DIALOG_CONTENT_OPTION_LABEL_CREATE = 'Create new for this resource';
    public const DIALOG_CONTENT_OPTION_LABEL_IDENTITY_NAME = '#%d (%s)';
    public const DIALOG_CONTENT_OPTION_LABEL_IDENTITY_NAME_DEFAULT = '#%d (no name)';
    public const DIALOG_CONTENT_OPTION_MARGIN = 8;
    public const DIALOG_CONTENT_SPACING = 1;

    public const DIALOG_CONTENT_OPTION_NAME_ALIGNMENT = 0.5;
    public const DIALOG_CONTENT_OPTION_NAME_MARGIN = 12;
    public const DIALOG_CONTENT_OPTION_NAME_PLACEHOLDER = 'Local name (optional)';

    // Extras
    private array $_options = [];

    public function __construct(
        Page $page,
    ) {
        // Init dependencies
        $this->page = $page;

        // Init requirements
        $this->name = new GtkEntry;

        $this->name->set_alignment(
            $this::DIALOG_CONTENT_OPTION_NAME_ALIGNMENT
        );

        $this->name->set_placeholder_text(
            _($this::DIALOG_CONTENT_OPTION_NAME_PLACEHOLDER)
        );

        $this->name->set_margin_start(
            $this::DIALOG_CONTENT_OPTION_NAME_MARGIN
        );

        $this->name->set_margin_end(
            $this::DIALOG_CONTENT_OPTION_NAME_MARGIN
        );

        $this->name->set_margin_bottom(
            $this::DIALOG_CONTENT_OPTION_NAME_MARGIN
        );
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

        // Init new certificate option
        $this->_options[0] = $this->_option(
            _($this::DIALOG_CONTENT_OPTION_LABEL_CREATE)
        );

        $this->_options[0]->join_group(
            $this->_options[0]
        );

        // Search database for auth records
        foreach ($this->page->container->browser->database->auth->like(
            sprintf(
                '%s%%',
                $this->page->navbar->request->getValue()
            )
        ) as $auth)
        {
            // Get related identity records
            if ($identity = $this->page->container->browser->database->identity->get($auth->identity))
            {
                $this->_options[$identity->id] = $this->_option(
                    $identity->name ? sprintf(
                        _($this::DIALOG_CONTENT_OPTION_LABEL_IDENTITY_NAME),
                        $identity->id,
                        $identity->name
                    ) : sprintf(
                        _($this::DIALOG_CONTENT_OPTION_LABEL_IDENTITY_NAME_DEFAULT),
                        $identity->id
                    )
                );

                $this->_options[$identity->id]->join_group(
                    $this->_options[0]
                );
            }
        }

        // Build options list
        foreach ($this->_options as $id => $option)
        {
            // Append option
            $content->add(
                $option
            );

            // Append name entry after new identity option
            if (!$id)
            {
                $content->add(
                    $this->name,
                    true,
                    true,
                    0
                );
            }
        }

        // Render
        $this->gtk->show_all();

        // Listen for user chose
        if (GtkResponseType::OK == $this->gtk->run())
        {
            // Get active option
            foreach ($this->_options as $id => $option)
            {
                // Auth
                if ($id)
                {
                    // @TODO activate existing record
                }

                // Generate new identity
                else
                {
                    // Detect driver
                    switch (true)
                    {
                        case mb_strtolower(
                            parse_url(
                                $this->page->navbar->request->getValue(),
                                PHP_URL_SCHEME
                            )
                        ) == 'gemini':

                            // Init identity model
                            $identity = new Gemini;

                            // Add new auth record
                            $this->page->container->browser->database->auth->add(
                                $this->page->container->browser->database->identity->add(
                                    $identity->crt(),
                                    $identity->key(),
                                    $this->name->get_text() ? $this->name->get_text() : null
                                ),
                                $this->page->navbar->request->getValue()
                            );

                        break;

                        default:

                            throw new Exception;
                    }
                }
            }

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
        $option = new GtkRadioButton;

        $option->set_label(
            $label
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

        $option->connect(
            'toggled',
            function (
                GtkRadioButton $self
            ) {
                // Detect active option
                foreach ($this->_options as $id => $option)
                {
                    // Is new
                    if (!$id)
                    {
                        if ($option->get_active())
                        {
                            $this->name->show();
                        }

                        else
                        {
                            $this->name->hide();
                        }
                    }
                }
            }
        );

        return $option;
    }
}