<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Tab\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Response\Query;
use \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Response\Send;

use \Yggverse\Net\Address;

class Response
{
    public \GtkBox $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page $page;

    // Requirements
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Response\Query $query;
    public \Yggverse\Yoda\Entity\Browser\Container\Tab\Page\Response\Send $send;

    // Defaults
    private int $_margin = 8;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Tab\Page $page
    ) {
        // Init dependencies
        $this->page = $page;

        // Init container
        $this->gtk = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->gtk->set_margin_top(
            $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $this->_margin
        );

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->gtk->set_spacing(
            $this->_margin
        );

        // Init query field
        $this->query = new Query(
            $this
        );

        $this->gtk->pack_start(
            $this->query->gtk,
            true,
            true,
            0
        );

        // Init send button
        $this->send = new Send(
            $this
        );

        $this->gtk->add(
            $this->send->gtk
        );

        // Hide widget by default
        $this->hide();
    }

    public function show(): void
    {
        $this->gtk->show_all();
    }

    public function hide(): void
    {
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