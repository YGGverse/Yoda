<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Auth\Option;

use \GtkRadioButton;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Auth;

use \Yggverse\Yoda\Model\Identity\Gemini;

class Identity
{
    // GTK
    public GtkRadioButton $gtk;

    // Dependencies
    public Auth $auth;

    // Requirements
    public ?Identity\Name $name = null;

    // Defaults
    public const MARGIN = 12;
    public const LABEL_DEFAULT = '#%d (%s)';
    public const LABEL_NO_NAME = '#%d (no name)';
    public const LABEL_CRT_NEW = 'Create new for this resource';

    public function __construct(
        Auth $auth
    ) {
        // Init dependencies
        $this->auth = $auth;

        // Init GTK
        $this->gtk = new GtkRadioButton;

        $this->gtk->set_margin_top(
            $this::MARGIN
        );

        $this->gtk->set_margin_start(
            $this::MARGIN
        );

        $this->gtk->set_margin_end(
            $this::MARGIN
        );

        $this->gtk->show();

        // Connect events
        $this->gtk->connect(
            'toggled',
            function(): void
            {
                $this->auth->refresh();
            }
        );
    }

    public function setGroup(
        Identity $identity
    ): void
    {
        $this->gtk->join_group(
            $identity->gtk
        );
    }

    public function setLabel(
        int $id,
        ?string $label = null
    ): void
    {
        if ($id)
        {
            $this->gtk->set_label(
                $label ? sprintf(
                    $this::LABEL_DEFAULT,
                    $id,
                    $label
                ) : sprintf(
                    $this::LABEL_NO_NAME,
                    $id
                )
            );
        }

        else
        {
            $this->gtk->set_label(
                $this::LABEL_CRT_NEW
            );
        }
    }

    public function setName(
        string $label
    ): void
    {
        $this->name = new Identity\Name(
            $this
        );
    }
}