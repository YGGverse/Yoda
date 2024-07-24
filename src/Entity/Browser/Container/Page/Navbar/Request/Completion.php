<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Request;

use \GtkEntryCompletion;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Request;

class Completion
{
    // GTK
    public GtkEntryCompletion $gtk;

    // Defaults
    public const INLINE_COMPLETION = true;
    public const INLINE_SELECTION = true;
    public const MINIMUM_KEY_LENGTH = 1; // @TODO
    public const TEXT_COLUMN = 0;

    // Dependencies
    public Request $request;

    // Requirements
    public Completion\Suggestion $suggestion;

    public function __construct(
        Request $request
    ) {
        // Dependencies
        $this->request = $request;

        // GTK
        $this->gtk = new GtkEntryCompletion;

        $this->gtk->set_inline_completion(
            $this::INLINE_COMPLETION
        );

        $this->gtk->set_inline_selection(
            $this::INLINE_SELECTION
        );

        $this->gtk->set_minimum_key_length(
            $this::MINIMUM_KEY_LENGTH
        );

        $this->gtk->set_text_column(
            $this::TEXT_COLUMN
        );

        // Requirements
        $this->suggestion = new Completion\Suggestion(
            $this
        );

        $this->gtk->set_model(
            $this->suggestion->gtk
        );
    }

    public function refresh(
        int $limit = 5,
        int $offset = 0
    ): void
    {
        $this->suggestion->clear();

        foreach ($this->request->navbar->page->container->browser->database->findHistory(
            $this->request->getValue(),
            $offset,
            $limit
        ) as $history)
        {
            $this->suggestion->append(
                $history->url
            );
        }
    }
}