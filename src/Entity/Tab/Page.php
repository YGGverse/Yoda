<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Tab;

class Page
{
    public \Yggverse\Yoda\Entity\App $app;

    public \Yggverse\Yoda\Model\Memory $dns;
    public \Yggverse\Yoda\Model\History $history;

    public \GtkBox $box,
                   $header,
                   $body,
                   $footer;

    public \GtkButton $home,
                      $back,
                      $forward,
                      $base,
                      $go;

    public \GtkEntry $request;

    public \GtkLabel $content,
                     $status;

    public \GtkScrolledWindow $container;

    public \GtkProgressBar $progressbar;

    public \GtkEntryCompletion $completion;

    public \GtkListStore $suggestion;

    public object $config;

    public function __construct(
        \Yggverse\Yoda\Entity\App $app
    ) {
        // Init app
        $this->app = $app;

        // Init config
        $this->config = \Yggverse\Yoda\Model\File::getConfig()->app->tab->page;

        // Init DNS memory
        $this->dns = new \Yggverse\Yoda\Model\Memory;

        // Init history
        $this->history = new \Yggverse\Yoda\Model\History;

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

        // Init home button
        $this->home = \GtkButton::new_with_label(
            $this->config->header->button->home->label
        );

        if ($this->config->header->button->home->visible)
        {
            $this->home->connect(
                'clicked',
                function ($entry)
                {
                    $this->history->reset();

                    $this->open(
                        $this->config->header->button->home->url
                    );
                }
            );

            $this->header->add(
                $this->home
            );
        }

        // Init back button
        $this->back = \GtkButton::new_with_label(
            $this->config->header->button->back->label
        );

        // Init forward button
        $this->forward = \GtkButton::new_with_label(
            $this->config->header->button->forward->label
        );

        // Group back/forward buttons
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
                $this->back->set_sensitive(
                    false
                );

                $this->back->connect(
                    'clicked',
                    function ($entry)
                    {
                        $this->open(
                            $this->history->goBack(),
                            false
                        );
                    }
                );

                $buttonGroup->add(
                    $this->back
                );
            }

            if ($this->config->header->button->forward->visible)
            {
                $this->forward->set_sensitive(
                    false
                );

                $this->forward->connect(
                    'clicked',
                    function ($entry)
                    {
                        $this->open(
                            $this->history->goForward(),
                            false
                        );
                    }
                );

                $buttonGroup->add(
                    $this->forward
                );
            }

            $this->header->add(
                $buttonGroup
            );
        }

        // Init base button
        $this->base = \GtkButton::new_with_label(
            $this->config->header->button->base->label
        );

        $this->base->set_sensitive(
            false
        );

        if ($this->config->header->button->base->visible)
        {
            $this->base->connect(
                'clicked',
                function (\GtkButton $button)
                {
                    $base = new \Yggverse\Net\Address(
                        $this->request->get_text()
                    );

                    $this->open(
                        $base->get(
                            true,  // scheme
                            true,  // user
                            true,  // pass
                            true,  // host
                            true,  // port
                            false, // path
                            false, // query
                            false  // fragment
                        )
                    );
                }
            );

            $this->header->add(
                $this->base
            );
        }

        // Request field
        $this->request = new \GtkEntry;

        $this->request->set_placeholder_text(
            $this->config->header->entry->request->placeholder
        );

        $this->request->set_max_length(
            $this->config->header->entry->request->length->max
        );

        $this->header->pack_start(
            $this->request,
            true,
            true,
            0
        );

        $this->request->connect(
            'activate',
            function ($entry)
            {
                $this->open(
                    $entry->get_text()
                );
            }
        );

        // Init autocomplete
        if ($this->config->header->entry->request->autocomplete->enabled)
        {
            $this->completion = new \GtkEntryCompletion();

            $this->completion->set_inline_completion(
                $this->config->header->entry->request->autocomplete->inline->completion
            );

            $this->completion->set_inline_selection(
                $this->config->header->entry->request->autocomplete->inline->selection
            );

            $this->completion->set_minimum_key_length(
                $this->config->header->entry->request->autocomplete->key->length
            );

            $this->completion->set_text_column(
                0
            );

            $this->suggestion = new \GtkListStore(
                \GObject::TYPE_STRING
            );

            $this->completion->set_model(
                $this->suggestion
            );

            $this->request->connect(
                'key-release-event',
                function ($entry, $event)
                {
                    if (
                        mb_strlen($entry->get_text()) >= $this->config->header->entry->request->autocomplete->key->length
                        &&
                        isset($event->key->keycode)
                        &&
                        !in_array(
                            $event->key->keycode,
                            $this->config->header->entry->request->autocomplete->ignore->keycode
                        )
                    ) {
                        $this->suggestion->clear();

                        foreach ($this->app->database->getHistory(
                            $entry->get_text(), 0, $this->config->header->entry->request->autocomplete->result->limit
                        ) as $suggestion)
                        {
                            $this->suggestion->append(
                                [
                                    $suggestion->url
                                ]
                            );
                        }

                        $this->request->set_completion(
                            $this->completion
                        );
                    }
                }
            );
        }

        // Go button
        $this->go = \GtkButton::new_with_label(
            $this->config->header->button->go->label
        );

        if ($this->config->header->button->go->visible)
        {
            $this->go->connect(
                'clicked',
                function ($entry)
                {
                    $this->open(
                        $this->request->get_text()
                    );
                }
            );

            $this->header->add(
                $this->go
            );
        }

        // Compose body
        $this->content = new \GtkLabel;

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
        $this->container = new \GtkScrolledWindow;

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

        $this->content->connect(
            'activate-link',
            function ($label, $href)
            {
                $address = new \Yggverse\Net\Address(
                    $href
                );

                if ($address->isRelative())
                {
                    $base = new \Yggverse\Net\Address(
                        $this->request->get_text()
                    );

                    if ($absolute = $address->getAbsolute($base))
                    {
                        $this->open(
                            $absolute
                        );
                    }

                    else
                    {
                        throw new Exception(); // @TODO
                    }
                }

                else
                {
                    $this->open(
                        $address->get()
                    );
                }
            }
        );

        // Init progressbar
        $this->progressbar = new \GtkProgressBar();

        $this->progressbar->set_opacity(
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

        $this->status = new \GtkLabel;

        $this->status->connect(
            'activate-link',
            function ($label, $href)
            {
                $this->open(
                    $href
                );
            }
        );

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
            $this->progressbar
        );

        $this->box->add(
            $this->footer
        );
    }

    public function open(
        string $url,
        bool $history = true,
        int $code = 0
    ): void
    {
        // Filter URL
        $url = trim(
            $url
        );

        // Update history in memory pool
        if ($history && $this->config->history->memory->enabled && $url != $this->history->getCurrent())
        {
            $this->history->add(
                $url
            );
        }

        // Update home button sensibility on match requested
        if ($this->config->header->button->home->visible)
        {
            $this->home->set_sensitive(
                !($url == $this->config->header->button->home->url)
            );
        }

        // Update back button sensibility
        if ($this->config->header->button->back->visible)
        {
            $this->back->set_sensitive(
                (bool) $this->history->getBack()
            );
        }

        // Update forward button sensibility
        if ($this->config->header->button->forward->visible)
        {
            $this->forward->set_sensitive(
                (bool) $this->history->getForward()
            );
        }

        // Update base button sensibility
        if ($this->config->header->button->base->visible)
        {
            // Update address
            $base = new \Yggverse\Net\Address(
                $this->request->get_text()
            );

            $this->base->set_sensitive(
                !($url == $base->get(
                    true,  // scheme
                    true,  // user
                    true,  // pass
                    true,  // host
                    true,  // port
                    false, // path
                    false, // query
                    false  // fragment
                ))
            );
        }

        // Update request field by requested
        $this->request->set_text(
            $url
        );

        // Open current address
        switch (true)
        {
            case str_starts_with($url, 'gemini://'):

                $this->_openGemini(
                    $url,
                    $code
                );

            break;

            case str_starts_with($url, 'nex://'):

                $this->_openNex(
                    $url
                );

            break;

            case str_starts_with($url, 'yoda://'):

                $this->_openYoda(
                    $url
                );

            break;

            default:

                $this->_openYoda(
                    'yoda://protocol'
                );
        }
    }

    private function _openGemini(
        string $url,
        int $code = 0,
        int $redirects = 0,
        bool $history = true
    ): void
    {
        // Init progressbar
        if ($this->config->progressbar->visible)
        {
            $this->setProgress(0);
        }

        // Init base URL
        $origin = new \Yggverse\Net\Address(
            $url
        );

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

            if (!$host = $this->dns->get($name))
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

                    $this->dns->set(
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

        // Process redirect
        if (in_array($response->getCode(), $this->config->redirect->follow->code))
        {
            if ($this->config->redirect->follow->enabled)
            {
                if ($redirects > $this->config->redirect->follow->max)
                {
                    $this->_openYoda(
                        'yoda://redirect'
                    );

                    return;
                }

                $redirect = new \Yggverse\Net\Address(
                    $url
                );

                $redirect->setPath(
                    $response->getMeta()
                );

                $this->open(
                    $redirect->get(),
                    false,
                    $response->getCode(),
                    $redirects + 1
                );

                return;
            }

            else
            {
                $this->_openYoda(
                    'yoda://redirect'
                );

                return;
            }
        }

        // Process error codes
        if (20 != $response->getCode()) // not found
        {
            $this->_openYoda(
                'yoda://nothing'
            );

            return;
        } // @TODO other codes

        $this->content->set_markup(
            \Yggverse\Gemini\Gtk3\Pango::fromGemtext(
                $response->getBody()
            )
        );

        $body = new \Yggverse\Gemini\Gemtext\Body(
            $response->getBody()
        );

        // Try to detect document title
        if ($h1 = $body->getH1())
        {
            $title = reset(
                $h1
            );
        }

        else if ($h2 = $body->getH2())
        {
            $title = reset(
                $h2
            );
        }

        else if ($h3 = $body->getH3())
        {
            $title = reset(
                $h3
            );
        }

        else
        {
            $title = $origin->getHost();
        }

        $this->setTitle(
            $title
        );

        $this->status->set_markup(
            str_replace( // Custom macros mask from config.json
                [
                    '{TIME_C}',
                    '{REQUEST_BASE}',
                    '{REQUEST_BASE_URL}',
                    '{RESPONSE_CODE}',
                    '{RESPONSE_META}',
                    '{RESPONSE_LENGTH}',
                    '{RESPONSE_SECONDS}'
                ],
                [
                    date(
                        'c'
                    ),
                    $origin->getHost(),
                    sprintf(
                        '<a href="%s">%s</a>',
                        $origin->get(
                            true,  // scheme
                            true,  // user
                            true,  // pass
                            true,  // host
                            true,  // port
                            false, // path
                            false, // query
                            false  // fragment
                        ),
                        $origin->getHost()
                    ),
                    $response->getCode(),
                    ($code ? sprintf('%d:', $code) : '')
                    .
                    ($response->getMeta() ? $response->getMeta() : $response->getCode()),
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

        // Update history database
        if ($history && $this->config->history->database->enabled)
        {
            $this->app->history->add(
                $url,
                $title,
                $this->config->history->database->mode->renew
            );
        }
    }

    private function _openNex(
        string $url,
        bool $history = true
    ): void
    {
        // Init progressbar
        if ($this->config->progressbar->visible)
        {
            $this->setProgress(0);
        }

        // Init base URL
        $origin = new \Yggverse\Net\Address(
            $url
        );

        // Track response time
        $start = microtime(true);

        // @TODO custom resolver support

        $client = new \Yggverse\Nex\Client;

        $response = $client->request(
            $url
        );

        $end = microtime(true);

        $this->content->set_markup(
            $response
        );

        $this->setTitle(
            $origin->getHost()
        );

        $this->status->set_markup(
            str_replace( // Custom macros mask from config.json
                [
                    '{TIME_C}',
                    '{REQUEST_BASE}',
                    '{REQUEST_BASE_URL}',
                    '{RESPONSE_CODE}',
                    '{RESPONSE_META}',
                    '{RESPONSE_LENGTH}',
                    '{RESPONSE_SECONDS}'
                ],
                [
                    date(
                        'c'
                    ),
                    $origin->getHost(),
                    sprintf(
                        '<a href="%s">%s</a>',
                        $origin->get(
                            true,  // scheme
                            true,  // user
                            true,  // pass
                            true,  // host
                            true,  // port
                            false, // path
                            false, // query
                            false  // fragment
                        ),
                        $origin->getHost()
                    ),
                    '-', // @TODO
                    '-',
                    number_format(
                        mb_strlen(
                            $response
                        )
                    ),
                    round(
                        $end - $start, 2
                    )
                ],
                $this->config->footer->status->open->complete
            )
        );

        // Update history database
        if ($history && $this->config->history->database->enabled)
        {
            $this->app->history->add(
                $url,
                $title,
                $this->config->history->database->mode->renew
            );
        }
    }

    private function _openYoda(
        string $url
    ): void
    {
        // Load local page
        if (!$data = \Yggverse\Yoda\Model\Page::get(str_replace('yoda://', '', $url)))
        {
            $data = \Yggverse\Yoda\Model\Page::get('Nothing');
        }

        $this->content->set_markup(
            \Yggverse\Gemini\Gtk3\Pango::fromGemtext(
                $data
            )
        );

        // Parse gemtext
        $body = new \Yggverse\Gemini\Gemtext\Body(
            $data
        );

        if ($h1 = $body->getH1())
        {
            $title = reset(
                $h1
            );

            $this->setTitle(
                $title
            );
        }
    }

    public function setTitle(
        ?string $value = null
    ): void
    {
        // Append hostname postfix
        if ($this->config->title->postfix->hostname && str_starts_with($this->request->get_text(), 'gemini://'))
        {
            $address = new \Yggverse\Net\Address(
                $this->request->get_text()
            );

            if ($address->getHost())
            {
                $value = sprintf(
                    '%s - %s',
                    $value,
                    $address->getHost()
                );
            }
        }

        // Build new tab label on title length reached
        if ($value && mb_strlen($value) > $this->config->title->width->chars)
        {
            $label = new \GtkLabel(
                $value ? $value : $this->config->title->default
            );

            if ($this->config->title->width->chars)
            {
                $label->set_width_chars(
                    $this->config->title->width->chars
                );
            }

            if ($this->config->title->ellipsize->mode)
            {
                $label->set_ellipsize(
                    // https://docs.gtk.org/Pango/enum.EllipsizeMode.html
                    $this->config->title->ellipsize->mode
                );
            }

            $this->app->tabs->set_tab_label(
                $this->box,
                $label
            );
        }

        else
        {
            $this->app->tabs->set_tab_label_text(
                $this->box,
                $value
            );
        }

        // Update window title
        $this->app->setTitle(
            $value ? $value : $this->config->title->default
        );
    }

    public function setProgress(
        float $value
    ): void
    {
        $this->progressbar->set_fraction(
            $value
        );

        \Gtk::timeout_add(
            10,
            function()
            {
                $progress = $this->progressbar->get_fraction();

                $progress = $progress + 0.02;

                $this->progressbar->set_fraction(
                    $progress
                );

                if ($progress < 1)
                {
                    $this->progressbar->set_opacity(
                        1
                    );
                }

                else
                {
                    $this->progressbar->set_opacity(
                        0
                    );

                    return false;
                }
            }
        );
    }
}