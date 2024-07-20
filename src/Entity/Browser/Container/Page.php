<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Title;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Progressbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Response;

use \Yggverse\Yoda\Model\Connection;
use \Yggverse\Yoda\Model\Filesystem;

class Page
{
    public \GtkBox $gtk;

    // Dependencies
    public Container $container;

    // Requirements
    public Title $title;
    public Navbar $navbar;
    public Progressbar $progressbar;
    public Content $content;
    public Response $response;

    public function __construct(
        Container $container
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

    public function init(
        ?string $request = null,
        bool $focus = false
    ): void
    {
        if ($request)
        {
            $this->navbar->request->setValue(
                $request
            );
        }

        if ($focus)
        {
            \Gtk::timeout_add(
                100,
                function()
                {
                    $this->navbar->request->focus();

                    return false;
                }
            );
        }
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
        // Update title
        $this->title->set(
            _('Loading...')
        );

        // Refresh navbar
        $this->navbar->refresh();

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
            function() use ($connection, $expire, $history)
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
                    // Update title
                    $this->title->set(
                        $connection->getTitle(),
                        $connection->getSubtitle(),
                        $connection->getTooltip()
                    );

                    // Refresh header by new title if current page is active
                    if ($this === $this->container->tab->get())
                    {
                        $this->container->browser->header->setTitle(
                            $this->title->getValue(),
                            $this->title->getSubtitle()
                        );
                    }

                    // Show response form
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

                    // Refresh header by new title if current page is active
                    if ($this === $this->container->tab->get())
                    {
                        $this->container->browser->header->setTitle(
                            $this->title->getValue(),
                            $this->title->getSubtitle()
                        );
                    }

                    // Update content
                    $this->content->set(
                        $connection->getMime(),
                        $connection->getData()
                    );

                    // Hide progressbar
                    $this->progressbar->hide();

                    // Free shared memory pool
                    $connection->close();

                    // Update history
                    if ($history)
                    {
                        // Save request in memory
                        $this->navbar->history->add(
                            $this->navbar->request->getValue()
                        );

                        // Save request in database (on background)
                        $pid = pcntl_fork();

                        if ($pid === 0)
                        {
                            $this->container->browser->database->renewHistory(
                                $this->navbar->request->getValue(),
                                $this->title->getValue()
                            );

                            exit;
                        }
                    }

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

                    // Refresh header by new title if current page is active
                    if ($this === $this->container->tab->get())
                    {
                        $this->container->browser->header->setTitle(
                            $this->title->getValue()
                        );
                    }

                    // Update content
                    $this->content->set(
                        Filesystem::MIME_TEXT_PLAIN,
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