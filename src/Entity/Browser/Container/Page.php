<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Title;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Progressbar;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Response;

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

        // Update content by multi-protocol responser
        $response = new \Yggverse\Yoda\Model\Response(
            $this->navbar->request->getValue(),
            $timeout
        );

        // Calculate expiration time
        $expire = time() + $timeout;

        // Listen response
        \Gtk::timeout_add(
            $refresh,
            function() use ($response, $expire)
            {
                // Redirect requested
                if ($location = $response->getRedirect())
                {
                    $this->open(
                        $location
                    );

                    return false; // stop
                }

                // Update title
                $this->title->set(
                    $response->getTitle(),
                    $response->getSubtitle(),
                    $response->getTooltip()
                );

                // Update content
                switch ($response->getMime())
                {
                    case 'text/gemini':

                        $title = null;

                        $this->content->setGemtext(
                            (string) $response->getData(),
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
                            (string) $response->getData()
                        );

                    break;

                    default:

                        throw new \Exception(
                            _('MIME type not supported')
                        );
                }

                // Response form requested
                if ($request = $response->getRequest())
                {
                    $this->response->show(
                        $request['placeholder'],
                        $request['visible']
                    );
                }

                else $this->response->hide();

                // Stop event loop on request completed
                if ($response->isCompleted())
                {
                    // Hide progressbar
                    $this->progressbar->hide();

                    // Stop
                    return false;
                }

                // Stop event loop on request expired
                if (time() > $expire)
                {
                    // Hide progressbar
                    $this->progressbar->hide();

                    // Stop
                    return false;
                }
            }
        );
    }
}