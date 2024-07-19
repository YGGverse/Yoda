<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Response;

class Query extends \Yggverse\Yoda\Abstract\Entity\Entry
{
    public \Yggverse\Yoda\Entity\Browser\Container\Page\Response $response;

    // Defaults
    protected string $_placeholder = 'Enter response...';

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page\Response $response
    ) {
        // Use parent features
        parent::__construct();

        // Init dependency
        $this->response = $response;
    }

    protected function _onActivate(
        \GtkEntry $entry
    ): void
    {
        $this->response->send();
    }

    protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {
        $this->response->refresh();
    }

    protected function _onChanged(
        \GtkEntry $entry
    ): void
    {}

    public function refresh(): void
    {
        // @TODO
    }
}