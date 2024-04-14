<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Tab;

class History
{
    public \Yggverse\Yoda\Entity\App $app;

    public object $config;

    public function __construct(
        \Yggverse\Yoda\Entity\App $app
    ) {
        // Init app
        $this->app = $app;

        // Init config
        $this->config = \Yggverse\Yoda\Model\File::getConfig()->app->tab->page;
    }
}