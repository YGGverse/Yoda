<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Tab;

class Page
{
    public \Yggverse\Yoda\Model\Memory $memory;

    public \GtkBox $box,
                   $header,
                   $body,
                   $footer;

    public \GtkButton $home,
                      $back,
                      $forward,
                      $go;

    public \GtkEntry $address;

    public \GtkLabel $content,
                     $status;

    public \GtkScrolledWindow $container;

    public object $config;

    public function __construct(
        ?string $url = null
    ) {
        // Init config
        $this->config = \Yggverse\Yoda\Model\File::getConfig()->window->tab->page;

        // Init memory
        $this->memory = new \Yggverse\Yoda\Model\Memory();

        // Compose header
        $this->header = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->header->set_margin_top(
            $this->config->header->margin
        );

        $this->header->set_margin_bottom(
            $this->config->header->margin
        );

        $this->header->set_margin_start(
            $this->config->header->margin
        );

        $this->header->set_margin_end(
            $this->config->header->margin
        );

        $this->header->set_spacing(
            $this->config->header->margin
        );

        // Home button
        $this->home = \GtkButton::new_with_label(
            $this->config->header->button->home->label
        );

        $this->home->set_sensitive(
            !($url == $this->config->header->button->home->url)
        );

        $this->home->connect(
            'released',
            function ($entry)
            {
                $this->open(
                    $this->config->header->button->home->url
                );
            }
        );

        if ($this->config->header->button->home->visible)
        {
            $this->header->add(
                $this->home
            );
        }

        // Back button
        $this->back = \GtkButton::new_with_label(
            $this->config->header->button->back->label
        );

        // Forward button
        $this->forward = \GtkButton::new_with_label(
            $this->config->header->button->forward->label
        );

        /// Group buttons
        if ($this->config->header->button->back->visible || $this->config->header->button->forward->visible)
        {
            $buttonGroup = new \GtkButtonBox(
                \GtkOrientation::HORIZONTAL
            );

            $buttonGroup->set_layout(
                \GtkButtonBoxStyle::EXPAND
            );

            if ($this->config->header->button->back->visible)
            {
                $buttonGroup->add(
                    $this->back
                );
            }

            if ($this->config->header->button->forward->visible)
            {
                $buttonGroup->add(
                    $this->forward
                );
            }

            $this->header->add(
                $buttonGroup
            );
        }

        // Address field
        $this->address = new \GtkEntry();

        if ($url)
        {
            $this->address->set_text(
                $url
            );
        }

        $this->address->set_placeholder_text(
            $this->config->header->address->placeholder
        );

        $this->address->set_max_length(
            $this->config->header->address->length->max
        );

        $this->address->connect(
            'activate',
            function ($entry)
            {
                $this->open(
                    $entry->get_text()
                );
            }
        );

        $this->header->pack_start(
            $this->address,
            true,
            true,
            0
        );

        // Go button
        $this->go = \GtkButton::new_with_label(
            $this->config->header->button->go->label
        );

        $this->go->connect(
            'released',
            function ($entry)
            {
                $this->open(
                    $this->address->get_text()
                );
            }
        );

        if ($this->config->header->button->go->visible)
        {
            $this->header->add(
                $this->go
            );
        }

        // Compose body
        $this->content = new \GtkLabel();

        $this->content->set_use_markup(
            true
        );

        $this->content->set_selectable(
            true
        );

        $this->content->set_line_wrap(
            true
        );

        $this->content->set_xalign(
            0
        );

        $this->content->set_yalign(
            0
        );

        // Init scrolled container
        $this->container = new \GtkScrolledWindow();

        $this->container->add(
            $this->content
        );

        $this->body = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->body->set_margin_start(
            $this->config->body->margin
        );

        $this->body->pack_start(
            $this->container,
            true,
            true,
            0
        );

        // Compose footer
        $this->footer = new \GtkBox(
            \GtkOrientation::HORIZONTAL
        );

        $this->footer->set_margin_top(
            $this->config->footer->margin
        );

        $this->footer->set_margin_bottom(
            $this->config->footer->margin
        );

        $this->footer->set_margin_start(
            $this->config->footer->margin
        );

        $this->footer->set_margin_end(
            $this->config->footer->margin
        );

        $this->footer->set_spacing(
            $this->config->footer->margin
        );

        $this->status = new \GtkLabel();

        $this->footer->add(
            $this->status
        );

        // Compose page
        $this->box = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->box->add(
            $this->header
        );

        $this->box->pack_start(
            $this->body,
            true,
            true,
            0
        );

        $this->box->add(
            $this->footer
        );
    }

    public function open(
        string $url
    ): void
    {
        // Update address field by requested
        $this->address->set_text(
            $url
        );

        // Update home button sensitivity on match requested
        $this->home->set_sensitive(
            !($url == $this->config->header->button->home->url)
        );

        // Open current address
        switch (true)
        {
            case str_starts_with($url, 'gemini://'):

                $this->_gemini(
                    $url
                );

            break;

            default:

                $this->_yoda(
                    $url
                );
        }
    }

    private function _gemini(string $url): void
    {
        // Track response time
        $start = microtime(true);

        // Init custom resolver
        $host = null;

        if ($this->config->resolver->enabled)
        {
            $address = new \Yggverse\Net\Address(
                $url
            );

            $name = $address->getHost();

            if (!$host = $this->memory->get($name))
            {
                $resolve = new \Yggverse\Net\Resolve(
                    $this->config->resolver->request->record,
                    $this->config->resolver->request->host,
                    $this->config->resolver->request->timeout,
                    $this->config->resolver->result->shuffle
                );

                $resolved = $resolve->address(
                    $address
                );

                if ($resolved)
                {
                    $host = $resolved->getHost();

                    $this->memory->set(
                        $name,
                        $host
                    );
                }
            }
        }

        $request = new \Yggverse\Gemini\Client\Request(
            $url,
            $host
        );

        $raw = $request->getResponse();

        $end = microtime(true);

        $response = new \Yggverse\Gemini\Client\Response(
            $raw
        );

        $this->content->set_markup(
            $this->_gemtext(
                $response->getBody()
            )
        );

        /* @TODO
        $body = new \Yggverse\Gemini\Gemtext\Body(
            $response->getBody()
        );

        if ($h1 = $body->getH1())
        {
            $this->window->set_title(
                sprintf(
                    '%s - Yoda',
                    empty($h1[0]) ? $address->getHost() : $h1[0]
                )
            );
        }
        */

        $this->status->set_text(
            str_replace( // Custom macros mask from config.json
                [
                    '{NAVIGATION_ADDRESS}',
                    '{TIME_C}',
                    '{RESPONSE_META}',
                    '{RESPONSE_LENGTH}',
                    '{RESPONSE_SECONDS}'
                ],
                [
                    urlencode(
                        $url
                    ),
                    date(
                        'c'
                    ),
                    $response->getMeta() ? $response->getMeta() : $response->getCode(),
                    number_format(
                        mb_strlen(
                            $raw
                        )
                    ),
                    round(
                        $end - $start, 2
                    )
                ],
                $this->config->footer->status->open->complete
            )
        );
    }

    private function _yoda(
        string $url
    ): void
    {
        // Load local page
        if (!$data = \Yggverse\Yoda\Model\Page::get(str_replace('yoda://', '', $url)))
        {
            $data = \Yggverse\Yoda\Model\Page::get('Oops');
        }

        $this->content->set_markup(
            $this->_gemtext(
                $data
            )
        );

        // Parse gemtext
        /* @TODO
        $body = new \Yggverse\Gemini\Gemtext\Body(
            $data
        );

        if ($h1 = $body->getH1())
        {
            $this->window->set_title(
                $h1[0]
            );
        }
        */
    }

    private function _gemtext(
        string $gemtext
    ): string
    {
        // Format body
        $body = new \Yggverse\Gemini\Gemtext\Body(
            $gemtext
        );

        $lines = $body->getLines();

        $escaped = [];

        /// Format H1
        foreach ($body->getH1() as $index => $h1)
        {
            $lines[$index] = sprintf(
                '<span size="xx-large">%s</span>',
                htmlentities(
                    $h1
                )
            );

            $escaped[] = $index;
        }

        /// Format H2
        foreach ($body->getH2() as $index => $h2)
        {
            $lines[$index] = sprintf(
                '<span size="x-large">%s</span>',
                htmlentities(
                    $h2
                )
            );

            $escaped[] = $index;
        }

        /// Format H3
        foreach ($body->getH3() as $index => $h3)
        {
            $lines[$index] = sprintf(
                '<span size="large">%s</span>',
                htmlentities(
                    $h3
                )
            );

            $escaped[] = $index;
        }

        /// Escape entities
        foreach ($lines as $index => $line)
        {
            if (!in_array($index, $escaped))
            {
                $lines[$index] = htmlentities(
                    $line
                );
            }
        }

        // @TODO links, code, escape entities

        return implode(
            PHP_EOL,
            $lines
        );
    }
}