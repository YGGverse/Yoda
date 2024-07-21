<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Response;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Response;

class Send extends \Yggverse\Yoda\Abstract\Entity\Button
{
    // Dependencies
    public Response $response;

    // Defaults
    protected string $_label = 'Send';

    public function __construct(
        Response $response
    ) {
        // Use parent features
        parent::__construct();

        // Init dependency
        $this->response = $response;
    }

    protected function _onCLick(
        \GtkButton $entity
    ): void
    {
        $this->response->send();
    }

    public function refresh(): void
    {
        $this->gtk->set_sensitive(
            !empty($this->response->query->getValue())
        );
    }
}