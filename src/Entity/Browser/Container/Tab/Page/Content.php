<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content\Data;
use \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content\Viewport;

class Content
{
    public \GtkScrolledWindow $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page $page;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content\Data $data;
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Content\Viewport $viewport;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Tab\Page $page
    ) {
        $this->page = $page;

        // Init container
        $this->gtk = new \GtkScrolledWindow;

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $this->_margin
        );

        $this->gtk->set_propagate_natural_height( // instead of pack to parent
            true
        );

        $this->gtk->set_propagate_natural_width(
            true
        );

        // Init viewport
        // to integrate scrolled window features for data label
        $this->viewport = new Viewport(
            $this
        );

        // Init data label
        $this->data = new Data(
            $this
        );

        $this->viewport->gtk->add(
            $this->data->gtk
        );

        $this->gtk->add(
            $this->viewport->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function refresh()
    {
        // @TODO
    }

    public function update(
        bool $history = true
    ): void
    {
        // Parse address
        $address = new \Yggverse\Net\Address(
            $this->page->navbar->request->gtk->get_text()
        );

        // Init new title
        $this->page->title->setValue(
            $address->getHost(),
            'loading...'
        );

        if ($history)
        {
            // Remember address in the navigation memory
            $this->page->navbar->history->add(
                $address->get()
            );

            // Update history in database
            $this->page->tab->container->browser->database->renewHistory(
                $address->get(),
                // @TODO title
            );
        }

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

                            $this->data->setGemtext(
                                file_get_contents( // @TODO format relative links
                                    $address->getPath()
                                ),
                                $title
                            );

                            if ($title) // detect title by document h1
                            {
                                $this->page->title->setValue(
                                    $title,
                                    $this->page->title->subtitle
                                );
                            }

                        break;

                        default:

                            $this->page->title->setValue(
                                'Oops!',
                                'file extension not supported'
                            );

                            $this->data->setPlain(
                                'File extension not supported'
                            );
                    }
                }

                else
                {
                    $this->page->title->setValue(
                        'Failure',
                        'resource not found or not readable'
                    );

                    $this->data->setPlain(
                        'Could not open file'
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

                // Route status codes
                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
                switch ($response->getCode())
                {
                    case 10: // response expected

                        $this->page->title->setValue(
                            $address->getHost(),
                            $response->getMeta() ? $response->getMeta() : sprintf(
                                'response expected (code %d)',
                                intval(
                                    $response->getCode()
                                )
                            )
                        );

                        $this->page->response->show(
                            $response->getMeta() // pass to placeholder
                        );

                    break;

                    case 20: // ok

                        // Process content type
                        switch (true)
                        {
                            case str_contains($response->getMeta(), 'text/gemini'):

                                $title = null;

                                $this->data->setGemtext(
                                    $response->getBody(),
                                    $title
                                );

                                $this->page->title->setValue(
                                    $title ? $title : $address->getHost(), // detect title by document h1
                                    $response->getMeta()
                                );

                            break;

                            default:

                                $this->data->setPlain(
                                    $response->getBody()
                                );

                                $this->page->title->setValue(
                                    $address->getHost()
                                );
                        }

                    break;

                    case 31: // redirect @TODO

                        $this->data->setGemtext(
                            sprintf(
                                '=> %s',
                                $response->getMeta()
                            )
                        );

                        $this->page->title->setValue(
                            $address->getHost(),
                            sprintf(
                                'redirect (code %d)',
                                intval(
                                    $response->getCode()
                                )
                            )
                        );

                    break;

                    default:

                        $this->page->title->setValue(
                            'Failure',
                            sprintf(
                                'could not open resource (code %d)',
                                intval(
                                    $response->getCode()
                                )
                            )
                        );

                        $this->data->setPlain(
                            'Requested resource not available!'
                        );
                }

            break;

            case null:

                // Try gemini protocol
                $address = new \Yggverse\Net\Address(
                    sprintf(
                        'gemini://%s',
                        trim(
                            $this->page->navbar->request->gtk->get_text()
                        )
                    )
                );

                // Is hostname request
                if (filter_var(
                        $address->getHost(),
                        FILTER_VALIDATE_DOMAIN,
                        FILTER_FLAG_HOSTNAME
                    )
                ) {
                    $this->page->navbar->request->setValue(
                        $address->get()
                    );
                }

                // Is search request
                else
                {
                    $this->page->navbar->request->setValue(
                        sprintf(
                            'gemini://tlgs.one/search?%s', // @TODO custom provider
                            urlencode(
                                $this->page->navbar->request->gtk->get_text()
                            )
                        )
                    );
                }

                $this->update();

            return;

            default:

                $this->page->title->setValue(
                    'Oops!',
                    'protocol not supported!'
                );

                $this->data->setPlain(
                    'Protocol not supported!'
                );
        }

        // Render content
        $this->gtk->show_all();

        // Refresh page components
        $this->page->refresh();

        // Update window header
        $this->page->tab->container->browser->header->setTitle(
            $this->page->title->getValue(),
            $this->page->title->getSubtitle(),
        );
    }
}