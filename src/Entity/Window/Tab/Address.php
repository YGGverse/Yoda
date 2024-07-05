<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Window\Tab;

use \Yggverse\Yoda\Entity\Window\Tab\Address\Title;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Content;
use \Yggverse\Yoda\Entity\Window\Tab\Address\Statusbar;

class Address
{
    public \GtkBox $gtk;

    public \Yggverse\Yoda\Entity\Window\Tab $tab;

    public \Yggverse\Yoda\Entity\Window\Tab\Address\Title $title;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Navbar $navbar;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Content $content;
    public \Yggverse\Yoda\Entity\Window\Tab\Address\Statusbar $statusbar;

    public function __construct(
        \Yggverse\Yoda\Entity\Window\Tab $tab
    ) {
        $this->tab = $tab;

        $this->title = new Title(
            $this
        );

        $this->gtk = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->navbar = new Navbar(
            $this
        );

        $this->gtk->add(
            $this->navbar->gtk
        );

        $this->content = new Content(
            $this
        );

        $this->gtk->pack_start(
            $this->content->gtk,
            true,
            true,
            0
        );

        $this->statusbar = new Statusbar(
            $this
        );

        $this->gtk->add(
            $this->statusbar->gtk
        );
    }

    public function update(
        bool $history = true
    ): void
    {
        // Parse address
        $address = new \Yggverse\Net\Address(
            $this->navbar->request->gtk->get_text()
        );

        // Update title
        $this->title->setValue(
            $address->getHost()
        );

        // Update navbar elements
        $this->navbar->base->update(
            $address
        );

        if ($history)
        {
            // Remember address in the navigation memory
            $this->navbar->history->add(
                $address->get()
            );

            // Refresh history in database
            $this->navbar->address->tab->window->database->refreshHistory(
                $address->get(),
                // @TODO title
            );

            // Refresh tabs
            $this->navbar->address->tab->refresh();
        }

        // Update statusbar indicator
        $this->statusbar->setValue(
            'Loading...'
        );

        // Detect protocol
        switch ($address->getScheme())
        {
            case 'file':

                if (file_exists($address->getPath()) && is_readable($address->getPath()))
                {
                    switch ($address->getPath())
                    {
                        case is_dir($address->getPath()):

                            // @TODO build fs listing

                        break;

                        case str_ends_with($address->getPath(), '.gmi'):

                            $title = null;

                            $this->content->data->setValue(
                                file_get_contents( // @TODO format relative links
                                    $address->getPath()
                                ),
                                $title
                            );

                            if ($title) // detect title by document h1
                            {
                                $this->title->setValue(
                                    $title
                                );
                            }

                            $this->statusbar->setValue(
                                null
                            );

                        break;

                        default:

                            $this->title->setValue(
                                'Oops!'
                            );

                            $this->content->data->setValue(
                                'File extension not supported'
                            );

                            $this->statusbar->setValue(
                                null
                            );
                    }
                }

                else
                {
                    $this->title->setValue(
                        'Failure'
                    );

                    $this->content->data->setValue(
                        'Could not open file'
                    );

                    $this->statusbar->setValue(
                        'Resource not found or not readable'
                    );
                }

            break;

            case 'nex':

                // @TODO

            break;

            case 'gemini':

                $request = new \Yggverse\Gemini\Client\Request(
                    $address->get()
                );

                $response = new \Yggverse\Gemini\Client\Response(
                    $request->getResponse()
                );

                if (20 === $response->getCode())
                {
                    switch (true)
                    {
                        case str_contains($response->getMeta(), 'text/gemini'):

                            $title = null;

                            $this->content->data->setValue(
                                $response->getBody(),
                                $title
                            );

                            if ($title) // detect title by document h1
                            {
                                $this->title->setValue(
                                    $title
                                );
                            }

                        break;

                        default:

                            $this->content->data->setValue(
                                $response->getBody()
                            );
                    }

                    $this->statusbar->setValue(
                        $response->getMeta()
                    );
                }

                else
                {
                    $this->title->setValue(
                        'Failure'
                    );

                    $this->content->data->setValue(
                        sprintf(
                            'Resource not available (code %d)',
                            intval(
                                $response->getCode()
                            )
                        )
                    );

                    $this->statusbar->setValue(
                        'Request failed'
                    );
                }

            break;

            case null:

                // Try gemini protocol
                $address = new \Yggverse\Net\Address(
                    sprintf(
                        'gemini://%s',
                        $this->navbar->request->gtk->get_text()
                    )
                );

                // Hostname request
                if (filter_var(
                        $address->getHost(),
                        FILTER_VALIDATE_DOMAIN,
                        FILTER_FLAG_HOSTNAME
                    )
                ) {
                    $this->navbar->request->setValue(
                        $address->get()
                    );
                }

                // Search request
                else
                {
                    $this->navbar->request->setValue(
                        sprintf(
                            'gemini://tlgs.one/search?%s', // @TODO custom provider
                            urlencode(
                                $this->navbar->request->gtk->get_text()
                            )
                        )
                    );
                }

                $this->update();

            return;

            default:

                $this->title->setValue(
                    'Oops!'
                );

                $this->content->data->setValue(
                    sprintf(
                        'Protocol not supported',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->statusbar->setValue(
                    null
                );
        }

        $this->tab->window->header->setTitle(
            $this->title->gtk->get_text()
        );

        $this->gtk->show_all();
    }
}