<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar;

use \GtkButton;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Navbar\Button;

class Auth extends Button
{
    // Defaults
    public const IMAGE = 'avatar-default-symbolic';
    public const LABEL = 'Auth';
    public const TOOLTIP = 'Select identity';

    protected function _onCLick(
        GtkButton $entity
    ): void
    {
        // Show auth dialog
        if ($this->navbar->page->auth->dialog())
        {
            // Update page
            $this->navbar->page->update(
                false
            );
        }
    }

    public function refresh(): void
    {
        // Activate on feature supported by request protocol
        $this->gtk->set_sensitive(
            boolval(
                parse_url(
                    $this->navbar->request->getValue(),
                    PHP_URL_SCHEME
                ) == 'gemini'
            )
        );
    }
}