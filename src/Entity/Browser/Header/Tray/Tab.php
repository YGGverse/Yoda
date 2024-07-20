<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Header\Tray;

use \Yggverse\Yoda\Entity\Browser\Header\Tray;

class Tab
{
    public \GtkButton $gtk;

    // Dependencies
    public Tray $tray;

    // Defaults
    protected string $_label = '+';
    private string $_tooltip = 'New tab';

    public function __construct(
        Tray $tray
    ) {
        // Init dependency
        $this->tray = $tray;

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
                $this->tray->header->browser->container->tab->append(
                    null,
                    false
                );
            }
        );
    }
}