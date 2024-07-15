<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

/*
 * Single response API for multiple protocol providers
 *
 */
class Response
{
    // Subject
    private \Yggverse\Net\Address $_address;

    // Async status
    private bool $_completed = false;
    private bool $_expired = false;

    // Response
    private ?string $_title = null;
    private ?string $_subtitle = null;
    private ?string $_tooltip = null;
    private ?string $_mime = null;
    private ?string $_data = null;
    private ?string $_redirect = null;
    private ?array  $_request = null;

    // Config
    public const MIME_TEXT_GEMINI = 'text/gemini';
    public const MIME_TEXT_PLAIN = 'text/plain';

    public function __construct(
        string $request,
        int $timeout = 5
    ) {
        // Build address instance
        $this->_address = new \Yggverse\Net\Address(
            $request
        );

        // Detect protocol
        switch ($this->_address->getScheme())
        {
            case 'file':

                $this->_title = basename(
                    $this->_address->getPath()
                );

                $this->_subtitle = $this->_address->getPath();
                $this->_tooltip  = $this->_address->getPath();

                switch (true)
                {
                    case (
                        $list = \Yggverse\Yoda\Model\Filesystem::getList(
                            $this->_address->getPath()
                        )
                    ): // directory
                        $tree = [];

                        foreach ($list as $item)
                        {
                            $tree[] = trim(
                                sprintf(
                                    '=> file://%s %s',
                                    $item['path'],
                                    $item['name'] . (
                                        $item['file'] ? null : '/'
                                    )
                                )
                            );
                        }

                        $this->_mime = self::MIME_TEXT_GEMINI;

                        $this->_data = implode(
                            PHP_EOL,
                            $tree
                        ) . PHP_EOL;

                    break;

                    case file_exists(
                        $this->_address->getPath()
                    ) && is_readable(
                        $this->_address->getPath()
                    ):
                        $this->_data = strval(
                            file_get_contents(
                                $this->_address->getPath()
                            )
                        );

                        $this->_mime = strval(
                            mime_content_type(
                                $this->_address->getPath()
                            )
                        );

                        if ($this->_mime == self::MIME_TEXT_PLAIN)
                        {
                            $extension = pathinfo(
                                strval(
                                    $this->_address->getPath()
                                ),
                                PATHINFO_EXTENSION
                            );

                            if (in_array($extension, ['gmi', 'gemini']))
                            {
                                $this->_mime = self::MIME_TEXT_GEMINI;
                            }
                        }

                    break;

                    default:

                        $this->_title = _(
                            'Failure'
                        );

                        $this->_data  = _(
                            'Could not open location'
                        );
                }

                $this->_completed = true;

            break;

            case 'gemini': // @TODO async

                $request = new \Yggverse\Gemini\Client\Request(
                    $this->_address->get()
                );

                $response = new \Yggverse\Gemini\Client\Response(
                    $request->getResponse()
                );

                // Route status code
                // https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
                switch ($response->getCode())
                {
                    case 10: // response expected
                    case 11: // sensitive input

                        $this->_request =
                        [
                            'placeholder' => $response->getMeta(),
                            'visible'     => 11 !== $response->getCode()
                        ];

                    break;

                    case 20: // ok

                        $this->_data = $response->getBody();

                        switch (true)
                        {
                            case str_contains(
                                $response->getMeta(),
                                self::MIME_TEXT_GEMINI
                            ):

                                $this->_mime = self::MIME_TEXT_GEMINI;

                            break;

                            case str_contains(
                                $response->getMeta(),
                                self::MIME_TEXT_PLAIN
                            ):

                                $this->_mime = self::MIME_TEXT_PLAIN;

                            break;

                            default:

                                throw new \Exception(
                                    sprintf(
                                        _('MIME type not implemented for %s'),
                                        $response->getMeta()
                                    )
                                );
                        }

                        $this->_completed = true;

                    break;

                    case 31: // redirect
                             // show link, no follow

                        $this->_data = sprintf(
                            '=> %s',
                            $response->getMeta()
                        );

                        $this->_mime = self::MIME_TEXT_GEMINI;

                        $this->_completed = true;

                    break;

                    default:

                        $this->_title = _(
                            'Oops!'
                        );

                        $this->_data = sprintf(
                            'Could not open request (code: %d)',
                            intval(
                                $response->getCode()
                            )
                        );

                        $this->_mime = self::MIME_TEXT_GEMINI;

                        $this->_completed = true;
                }

            break;

            case 'nex': // @TODO async

                $this->_data = (
                    new \Yggverse\Nex\Client
                )->request(
                    $this->_address->get()
                );

                $this->_mime = self::MIME_TEXT_PLAIN; // @TODO

                $this->_completed = true;

            break;

            case null:

                // Build gemini protocol address
                $address = new \Yggverse\Net\Address(
                    sprintf(
                        'gemini://%s',
                        $this->_address->get()
                    )
                );

                // Validate hostname
                if (filter_var(
                        $address->getHost(),
                        FILTER_VALIDATE_DOMAIN,
                        FILTER_FLAG_HOSTNAME
                    )
                ) {
                    // Request redirect
                    $this->_redirect = $address->get();
                }

                // Request redirect to search provider
                else
                {
                    // @TODO custom providers
                    $this->_redirect = sprintf(
                        'gemini://tlgs.one/search?%s',
                        urlencode(
                            $request
                        )
                    );
                }

            return;

            default:

                throw new \Exception(
                    _('Protocol not supported')
                );
        }
    }

    public function isCompleted(): bool
    {
        return $this->_completed;
    }

    public function isExpired(): bool
    {
        return $this->_expired;
    }

    public function getTitle(): ?string
    {
        return $this->_title;
    }

    public function getSubtitle(): ?string
    {
        return $this->_subtitle;
    }

    public function getTooltip(): ?string
    {
        return $this->_tooltip;
    }

    public function getMime(): ?string
    {
        return $this->_mime;
    }

    public function getData(): ?string
    {
        return $this->_data;
    }

    public function getRedirect(): ?string
    {
        return $this->_redirect;
    }

    public function getRequest(): ?array
    {
        return $this->_request;
    }

    public function getLength(): ?int
    {
        return mb_strlen(
            $this->_data
        );
    }
}