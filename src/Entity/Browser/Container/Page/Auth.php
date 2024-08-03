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
    public const DEFAULT_RESPONSE = GtkResponseType::CANCEL;
    public const FORMAT_SECONDARY_TEXT = 'Select identity';
    public const MESSAGE_FORMAT = 'Authorization';

    public const MARGIN = 8;
    public const SPACING = 1;

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
            _($this::MESSAGE_FORMAT)
        );

        $this->gtk->format_secondary_text(
            _($this::FORMAT_SECONDARY_TEXT)
        );

        $this->gtk->set_default_response(
            $this::DEFAULT_RESPONSE
        );

        // Init content
        $content = $this->gtk->get_content_area();

        $content->set_spacing(
            $this::SPACING
        );

        // Init new certificate option
        $this->_options[
            Auth\Option\Identity::ID_CRT_NEW
        ] = new Auth\Option\Identity(
            $this
        );

        $this->_options[
            Auth\Option\Identity::ID_CRT_NEW
        ]->setGroup(
            $this->_options[
                Auth\Option\Identity::ID_CRT_NEW
            ]
        );

        $this->_options[
            Auth\Option\Identity::ID_CRT_NEW
        ]->setLabel(
            Auth\Option\Identity::ID_CRT_NEW
        );

        $this->_options[
            Auth\Option\Identity::ID_CRT_NEW
        ]->useName();

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
                    $this->_options[
                        Auth\Option\Identity::ID_CRT_NEW
                    ]
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
                    $option->name->gtk,
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
                    self::MARGIN
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
                                        $option->name->getValue()
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

            $this->destroy();

            return true;
        }

        // Dialog canceled
        $this->destroy();

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

    public function destroy(): void
    {
        // Free memory
        $this->_options = [];

        // Destroy GTK object
        $this->gtk->destroy();
    }
}