<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Title;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Progressbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Response;

use \Yggverse\Yoda\Model\Connection;

class Page
{
    public \GtkBox $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container $container;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Title $title;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar $navbar;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Progressbar $progressbar;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Content $content;
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Response $response;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container $container
    ) {
        // Init dependencies
        $this->container = $container;

        // Init container
        $this->gtk = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        // Init title
        $this->title = new Title(
            $this
        );

        // Init navbar
        $this->navbar = new Navbar(
            $this
        );

        $this->gtk->add(
            $this->navbar->gtk
        );

        // Init content
        $this->content = new Content(
            $this
        );

        $this->gtk->pack_start(
            $this->content->gtk,
            true,
            true,
            0
        );

        // Init progress bar
        $this->progressbar = new Progressbar(
            $this
        );

        $this->gtk->add(
            $this->progressbar->gtk
        );

        // Init response bar
        $this->response = new Response(
            $this
        );

        $this->gtk->pack_end(
            $this->response->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function refresh(): void
    {
        $this->navbar->refresh();
        $this->content->refresh();
    }

    public function open(
        ?string $request = null,
        bool $history = true
    ): void
    {
        $this->navbar->request->setValue(
            $request
        );

        $this->update(
            $history
        );
    }

    public function update(
        bool $history = true,
         int $refresh = 100,
         int $timeout = 5
    ): void
    {
        // Update history
        if ($history)
        {
            // Save request in memory
            $this->navbar->history->add(
                $this->navbar->request->getValue()
            );

            // Save request in database
            $this->container->browser->database->renewHistory(
                $this->navbar->request->getValue(),
                // @TODO title
            );
        }

        // Update title
        $this->title->set(
            _('Loading...')
        );

        // Show progressbar
        $this->progressbar->infinitive();

        // Hide response form
        $this->response->hide();

        // Update content using multi-protocol driver
        $connection = new Connection;

        // Async request
        $connection->request(
            $this->navbar->request->getValue(),
            $timeout
        );

        // Calculate expiration time
        $expire = time() + $timeout;

        // Listen response
        \Gtk::timeout_add(
            $refresh,
            function() use ($connection, $expire)
            {
                // Redirect requested
                if ($location = $connection->getRedirect())
                {
                    // Follow
                    $this->open(
                        $location
                    );

                    // Hide progressbar
                    $this->progressbar->hide();

                    // Free shared memory pool
                    $connection->close();

                    return false; // stop
                }

                // Response form requested
                if ($request = $connection->getRequest())
                {
                    $this->response->show(
                        $request['placeholder'],
                        $request['visible']
                    );

                    // Hide progressbar
                    $this->progressbar->hide();

                    // Free shared memory pool
                    $connection->close();

                    return false; // stop
                }

                // Stop event loop on request completed
                if ($connection->isCompleted())
                {
                    // Update title
                    $this->title->set(
                        $connection->getTitle(),
                        $connection->getSubtitle(),
                        $connection->getTooltip()
                    );

                    // Update content
                    switch ($connection->getMime())
                    {
                        case 'text/gemini':

                            $title = null;

                            $this->content->setGemtext(
                                (string) $connection->getData(),
                                $title
                            );

                            if ($title)
                            {
                                $this->title->setValue(
                                    $title
                                );
                            }

                        break;

                        case 'text/plain':

                            $this->content->setPlain(
                                (string) $connection->getData()
                            );

                        break;

                        /* @TODO
                        case 'image/gif':
                        case 'image/jpeg':
                        case 'image/png':
                        case 'image/webp':

                            $this->content->setImage(
                                (string) $connection->getData()
                            );

                        break;
                        */

                        default:

                            $this->title->setValue(
                                _('Oops!')
                            );

                            $this->content->setPlain(
                                _('MIME type not supported')
                            );
                    }

                    // Hide progressbar
                    $this->progressbar->hide();

                    // Free shared memory pool
                    $connection->close();

                    // Stop
                    return false;
                }

                // Stop event loop on request expired
                if (time() > $expire)
                {
                    // Update title
                    $this->title->set(
                        _('Timeout')
                    );

                    // Update content
                    $this->content->setGemtext(
                        _('Response time reached')
                    );

                    // Hide progressbar
                    $this->progressbar->hide();

                    // Free shared memory pool
                    $connection->close();

                    // Stop
                    return false;
                }
            }
        );
    }
}