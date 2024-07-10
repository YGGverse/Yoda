<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Data;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Viewport;

class Content
{
    public \GtkScrolledWindow $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Page $page;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Data $data;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Viewport $viewport;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page $page
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
            $this->page->navbar->request->getValue()
        );

        // Init new title
        $this->page->title->set(
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
            $this->page->container->browser->database->renewHistory(
                $address->get(),
                // @TODO title
            );
        }

        // Detect protocol
        switch ($address->getScheme())
        {
            case 'file':

                switch (true)
                {
                    // Try directory
                    case (
                        $list = \Yggverse\Yoda\Model\Filesystem::getList(
                            $address->getPath()
                        )
                    ):

                        $map = [];

                        foreach ($list as $item)
                        {
                            $map[] = trim(
                                sprintf(
                                    '=> file://%s %s',
                                    $item['path'],
                                    $item['name'] . (
                                        $item['file'] ? null : '/'
                                    )
                                )
                            );
                        }

                        $this->data->setGemtext(
                            implode(
                                PHP_EOL,
                                $map
                            ) . PHP_EOL
                        );

                        $this->page->title->set(
                            basename(
                                $address->getPath()
                            ),
                            'localhost'
                        );

                    break;

                    // Try open file by extension supported
                    case str_ends_with(
                        $address->getPath(),
                        '.gmi'
                    ):

                        $title = null;

                        $this->data->setGemtext(
                            file_get_contents( // @TODO format relative links
                                $address->getPath()
                            ),
                            $title
                        );

                        if ($title) // detect title by document h1
                        {
                            $this->page->title->set(
                                $title,
                                'localhost'
                            );
                        }

                        else
                        {
                            $this->page->title->set(
                                basename(
                                    $address->getPath()
                                ),
                                'localhost'
                            );
                        }

                    break;

                    default:

                        $this->page->title->set(
                            'Failure',
                            'resource not found or not readable'
                        );

                        $this->data->setPlain(
                            'Could not open location'
                        );
                }

            break;

            case 'nex':

                $client = new \Yggverse\Nex\Client;

                if ($response = $client->request($address->get()))
                {
                    // Detect content type
                    switch (true)
                    {
                        case in_array(
                            pathinfo(
                                strval(
                                    $address->getPath()
                                ),
                                PATHINFO_EXTENSION
                            ),
                            [
                                'gmi',
                                'gemini'
                            ]
                        ):

                            $title = null;

                            $this->data->setGemtext(
                                $response,
                                $title
                            );

                            $this->page->title->set(
                                $title ? sprintf(
                                    '%s - %s',
                                    $title,
                                    $address->getHost()
                                ) : $address->getHost()
                            );

                        break;

                        default:

                            $this->data->setMono(
                                $response
                            );

                            $this->page->title->set(
                                $address->getHost()
                            );
                    }
                }

                else
                {
                    $this->page->title->set(
                        'Failure',
                        'could not open resource'
                    );

                    $this->data->setPlain(
                        'Requested resource not available!'
                    );
                }

            break;

            case 'gemini':

                $request = new \Yggverse\Gemini\Client\Request(
                    $address->get()
                );

                $response = new \Yggverse\Gemini\Client\Response(
                    $request->getResponse()
                );

                // Route status code
                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
                switch ($response->getCode())
                {
                    case 10: // response expected
                    case 11: // sensitive input

                        $this->page->title->set(
                            $address->getHost(),
                            $response->getMeta() ? $response->getMeta() : 'response expected'
                        );

                        $this->page->response->show(
                            $response->getMeta(), // placeholder
                            boolval(11 !== $response->getCode()) // input visibility
                        );

                    break;

                    case 20: // ok

                        // Detect content type
                        switch (true)
                        {
                            case str_contains(
                                $response->getMeta(),
                                'text/gemini'
                            ):

                            case in_array(
                                pathinfo(
                                    strval(
                                        $address->getPath()
                                    ),
                                    PATHINFO_EXTENSION
                                ),
                                [
                                    'gmi',
                                    'gemini'
                                ]
                            ):

                                $title = null;

                                $this->data->setGemtext(
                                    $response->getBody(),
                                    $title
                                );

                                $this->page->title->set(
                                    $title ? sprintf(
                                        '%s - %s',
                                        $title,
                                        $address->getHost()
                                    ) : $address->getHost(), // detect title by document h1
                                    $response->getMeta()
                                );

                            break;

                            default:

                                $this->data->setPlain(
                                    $response->getBody()
                                );

                                $this->page->title->set(
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

                        $this->page->title->set(
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

                        $this->page->title->set(
                            'Failure',
                            sprintf(
                                '%s (code %d)',
                                $response->getMeta() ? $response->getMeta() : 'could not open resource',
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
                        $this->page->navbar->request->getValue()
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
                                $this->page->navbar->request->getValue()
                            )
                        )
                    );
                }

                $this->update();

            return;

            default:

                $this->page->title->set(
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
        $this->page->container->browser->header->setTitle(
            $this->page->title->getValue(),
            $this->page->title->getSubtitle(),
        );
    }
}