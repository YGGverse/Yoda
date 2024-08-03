<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Exception;
use \GtkButtonsType;
use \GtkDialogFlags;
use \GtkLabel;
use \GtkMessageDialog;
use \GtkMessageType;
use \GtkOrientation;
use \GtkRadioButton;
use \GtkResponseType;
use \GtkSeparator;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Model\Identity\Gemini;

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
    public const DIALOG_CONTENT_SPACING = 1;

    // Extras
    private array $_options = []; // GtkRadioButton

    public function __construct(
        Page $page
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

        // Init new certificate option
        $this->_options[0] = new Auth\Option\Identity(
            $this
        );

        $this->_options[0]->setGroup(
            $this->_options[0]
        );

        $this->_options[0]->setLabel(
            0, _($this::DIALOG_CONTENT_OPTION_LABEL_CREATE)
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
                $this->_options[$identity->id] = new Auth\Option\Identity(
                    $this
                );

                $this->_options[$identity->id]->setGroup(
                    $this->_options[0]
                );

                $this->_options[$identity->id]->setLabel(
                    $identity->id,
                    $identity->name
                );
            }
        }

        // Append separator
        $content->add(
            new GtkSeparator(
                GtkOrientation::VERTICAL
            )
        );

        // Build options list
        foreach ($this->_options as $id => $option)
        {
            // Append option
            $content->add(
                $option->gtk
            );

            // Is new and option has name entity
            if (!$id && !is_null($option->name))
            {
                // Append name entry after new identity option
                $content->add(
                    $option->name,
                    true,
                    true,
                    0
                );

                // Append separator
                $content->add(
                    new GtkSeparator(
                        GtkOrientation::VERTICAL
                    )
                );

                // Set margin
                $option->gtk->set_margin_bottom(
                    self::DIALOG_CONTENT_OPTION_MARGIN
                );
            }
        }

        // Append empty line separator
        $content->add(
            new GtkLabel
        );

        // Render
        $this->gtk->show_all();

        // Listen for user chose
        if (GtkResponseType::OK == $this->gtk->run())
        {
            // Find active option
            foreach ($this->_options as $id => $option)
            {
                if ($option->gtk->get_active())
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
                                        $this->_name->get_text() ? $this->_name->get_text() : null
                                    ),
                                    $this->page->navbar->request->getValue()
                                );

                            break;

                            default:

                                throw new Exception;
                        }
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

    public function refresh(): void
    {
        // Detect active option
        foreach ($this->_options as $id => $option)
        {
            // Is new and option has name entity
            if (!$id && !is_null($option->name))
            {
                // Update sensibility
                $option->name->gtk->set_sensitive(
                    $option->gtk->get_active()
                );

                break;
            }
        }
    }
}