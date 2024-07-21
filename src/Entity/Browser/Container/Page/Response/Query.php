<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Response;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Response;

class Query extends \Yggverse\Yoda\Abstract\Entity\Entry
{
    public Response $response;

    // Defaults
    public const PLACEHOLDER = 'Enter your response...';

    public function __construct(
        Response $response
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

    protected function _onFocusOut(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void
    {}

    public function refresh(): void
    {
        // @TODO
    }
}