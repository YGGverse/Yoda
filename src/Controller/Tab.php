<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Controller;

class Tab
{
    public \GtkWindow $window;

    public \Yggverse\Yoda\Entity\Box\Tab $tab;

    public \Yggverse\Yoda\Model\Memory $memory;

    public object $config;

    public function __construct(
        \GtkWindow $window
    ) {
        $this->window = $window;

        $this->config = \Yggverse\Yoda\Model\File::getConfig();

        $this->memory = new \Yggverse\Yoda\Model\Memory();

        $this->tab = new \Yggverse\Yoda\Entity\Box\Tab();

        $this->tab->navigation->address->entry->connect(
            'activate',
            function ($entry)
            {
                $this->navigate(
                    $entry->get_text()
                );
            }
        );

        $this->tab->navigation->go->button->connect(
            'released',
            function ($entry)
            {
                $this->navigate(
                    $this->tab->navigation->address->entry->get_text()
                );
            }
        );

        $this->tab->navigation->reload->button->connect(
            'released',
            function ($entry)
            {
                $this->navigate(
                    $this->tab->navigation->address->entry->get_text()
                );
            }
        );

        if ($this->config->homepage)
        {
            $this->tab->navigation->home->button->connect(
                'released',
                function ($entry)
                {
                    $this->tab->navigation->address->entry->set_text(
                        $this->config->homepage
                    );

                    $this->navigate(
                        $this->config->homepage
                    );
                }
            );
        }

        // @TODO back, forward buttons
    }

    public function navigate(string $url)
    {
        $this->tab->tray->label->set_text(
            sprintf(
                'Open %s...',
                urldecode(
                    $url
                )
            )
        );

        $start = microtime(true);

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

        $this->tab->content->label->set_markup(
            $response->getBody()
        );

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

        $this->tab->tray->label->set_text(
            sprintf(
                '%s | %s | %d bytes | %s seconds',
                date('c'),
                $response->getMeta() ? $response->getMeta() : $response->getCode(),
                number_format(
                    mb_strlen(
                        $raw
                    )
                ),
                round(
                    $end - $start, 2
                )
            )
        );
    }
}