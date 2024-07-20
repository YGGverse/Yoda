<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header;

use \Yggverse\Yoda\Entity\Browser\Header;

class Tab
{
    public \GtkButton $gtk;

    // Dependencies
    public Header $header;

    // Defaults
    protected string $_label = '+';
    private string $_tooltip = 'New tab';

    public function __construct(
        Header $header
    ) {
        // Init dependency
        $this->header = $header;

        // Init GTK
        $this->gtk = new \GtkButton;

        $this->gtk->set_label(
            _($this->_label)
        );

        $this->gtk->set_tooltip_text(
            _($this->_tooltip)
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'clicked',
            function(
                \GtkButton $entity
            ) {
                $this->header->browser->container->tab->append(
                    null,
                    false
                );
            }
        );
    }
}