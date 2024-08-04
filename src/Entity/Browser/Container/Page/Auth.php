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

use \Yggverse\Net\Address;

class Auth
{
    // GTK
    public GtkMessageDialog $gtk;

    // Dependencies
    public Page $page;

    // Defaults
    public const DEFAULT_RESPONSE = GtkResponseType::CANCEL;
    public const FORMAT_SECONDARY_TEXT = 'Select identity';
    public const MESSAGE_FORMAT = 'Auth';

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

        // Add separator
        $content->add(
            new GtkSeparator(
                GtkOrientation::VERTICAL
            )
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

        // Build records from database
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
                $this->_options[
                    $identity->id
                ] = new Auth\Option\Identity(
                    $this
                );

                $this->_options[
                    $identity->id
                ]->setGroup(
                    $this->_options[
                        Auth\Option\Identity::ID_CRT_NEW
                    ]
                );

                $this->_options[
                    $identity->id
                ]->setLabel(
                    $identity->id,
                    $identity->name
                );
            }
        }

        // Append logout option
        $this->_options[
            Auth\Option\Identity::ID_LOG_OUT
        ] = new Auth\Option\Identity(
            $this
        );

        $this->_options[
            Auth\Option\Identity::ID_LOG_OUT
        ]->setGroup(
            $this->_options[
                Auth\Option\Identity::ID_CRT_NEW
            ]
        );

        $this->_options[
            Auth\Option\Identity::ID_LOG_OUT
        ]->setLabel(
            Auth\Option\Identity::ID_LOG_OUT
        );

        // Build options list
        foreach ($this->_options as $id => $option)
        {
            // Detect option type
            switch ($id)
            {
                // Is new cert option
                case Auth\Option\Identity::ID_CRT_NEW:

                    // Set extra margin
                    $option->gtk->set_margin_bottom(
                        $option::MARGIN
                    );

                    // Append option
                    $content->add(
                        $option->gtk
                    );

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

                break;

                // Is logout option
                case Auth\Option\Identity::ID_LOG_OUT:

                    // Set extra margin
                    $option->gtk->set_margin_bottom(
                        $option::MARGIN
                    );

                    // Append option
                    $content->add(
                        $option->gtk
                    );

                break;

                // Is DB
                default:

                    // Append option
                    $content->add(
                        $option->gtk
                    );

                    // Detect active option match identity driver conditions
                    switch (true)
                    {
                        case parse_url(
                            $this->page->navbar->request->getValue(),
                            PHP_URL_SCHEME
                        ) == 'gemini':

                            $option->gtk->set_active(
                                boolval(
                                    Gemini::match(
                                        new Address(
                                            $this->page->navbar->request->getValue()
                                        ),
                                        $this->page->container->browser->database
                                    )
                                )
                            );

                        break;
                    }
            }
        }

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
                    // Detect option type
                    switch ($id)
                    {
                        case Auth\Option\Identity::ID_CRT_NEW:

                            // Logout previous session
                            $this->page->container->browser->database->auth->logout(
                                $this->page->navbar->request->getValue()
                            );

                            // Detect identity driver
                            switch (true)
                            {
                                case parse_url(
                                    $this->page->navbar->request->getValue(),
                                    PHP_URL_SCHEME
                                ) == 'gemini':

                                    // Init identity model
                                    $identity = new Gemini;

                                    // Add new auth record
                                    $this->page->container->browser->database->auth->add(
                                        $this->page->container->browser->database->identity->add(
                                            $identity->crt(),
                                            $identity->key(),
                                            $option->name ? $option->name->getValue() : null
                                        ),
                                        $this->page->navbar->request->getValue()
                                    );

                                break;

                                default:

                                    throw new Exception;
                            }

                        break;

                        case Auth\Option\Identity::ID_LOG_OUT:

                            // Logout previous session
                            $this->page->container->browser->database->auth->logout(
                                $this->page->navbar->request->getValue()
                            );

                        break;

                        default:

                            // Logout previous session
                            $this->page->container->browser->database->auth->logout(
                                $this->page->navbar->request->getValue()
                            );

                            // Add new auth record
                            $this->page->container->browser->database->auth->add(
                                $id,
                                $this->page->navbar->request->getValue()
                            );
                    }

                    break;
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
            // Detect option type
            switch ($id)
            {
                case Auth\Option\Identity::ID_CRT_NEW:

                    // Is name entity defined
                    if (!is_null($option->name))
                    {
                        // Update sensibility
                        $option->name->gtk->set_sensitive(
                            $option->gtk->get_active()
                        );
                    }

                break;

                case Auth\Option\Identity::ID_LOG_OUT:

                    // Update sensibility
                    $option->gtk->set_sensitive(
                        boolval(
                            count(
                                $this->_options
                            ) > 2
                        )
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