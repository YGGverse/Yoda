<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Net\Address;

class Response
{
    public \GtkBox $gtk;

    // Dependencies
    public Page $page;

    // Requirements
    public Response\Query $query;
    public Response\Send $send;

    // Defaults
    public const MARGIN = 8;
    public const SPACING = 8;

    public function __construct(
        Page $page
    ) {
        // Init dependencies
        $this->page = $page;

        // Init container
        $this->gtk = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->gtk->set_margin_top(
            $this::MARGIN
        );

        $this->gtk->set_margin_bottom(
            $this::MARGIN
        );

        $this->gtk->set_margin_start(
            $this::MARGIN
        );

        $this->gtk->set_margin_end(
            $this::MARGIN
        );

        $this->gtk->set_spacing(
            $this::SPACING
        );

        // Init query field
        $this->query = new Response\Query(
            $this
        );

        $this->gtk->pack_start(
            $this->query->gtk,
            true,
            true,
            0
        );

        // Init send button
        $this->send = new Response\Send(
            $this
        );

        $this->gtk->add(
            $this->send->gtk
        );

        // Hide widget by default
        $this->hide();
    }

    public function show(
        ?string $placeholder = null,
        ?bool $visible = null,
        bool $focus = true
    ): void
    {
        if ($focus)
        {
            $this->query->focus();
        }

        if (!is_null($placeholder))
        {
            $this->query->setPlaceholder(
                $placeholder
            );
        }

        if (!is_null($visible))
        {
            $this->query->setVisible(
                $visible
            );
        }

        $this->gtk->show();
    }

    public function hide(): void
    {
        $this->query->setPlaceholder(
            null
        );

        $this->query->setVisible(
            null
        );

        $this->gtk->hide();
    }

    public function refresh()
    {
        $this->query->refresh();
        $this->send->refresh();
    }

    public function send(): void
    {
        $address = new Address(
            $this->page->navbar->request->getValue()
        );

        $address->setQuery(
            urlencode(
                trim(
                    strval(
                        $this->query->getValue()
                    )
                )
            )
        );

        $this->page->open(
            $address->get(),
            false // disable history recording
        );

        $this->hide();
    }
}