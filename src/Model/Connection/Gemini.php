<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Connection;

use \Yggverse\Gemini\Client\Request;
use \Yggverse\Gemini\Client\Response;
use \Yggverse\Net\Address;

use \Yggverse\Yoda\Model\Connection;
use \Yggverse\Yoda\Model\Filesystem;

class Gemini
{
    private Connection $_connection;

    public function __construct(
        Connection $connection
    ) {
        $this->_connection = $connection;
    }

    public function request(
        Address $address,
        int $timeout = 5
    ): void
    {
        $request = new Request(
            $address->get()
        );

        $response = new Response(
            $request->getResponse(
                $timeout
            )
        );

        // @TODO reset title, mime, data

        // Route status code
        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
        switch ($response->getCode())
        {
            case 10: // response expected
            case 11: // sensitive input

                $this->_connection->setTitle(
                    _('Pending...')
                );

                $this->_connection->setSubtitle(
                    $response->getMeta() ? $response->getMeta()
                                         : _('Response expected')
                );

                $this->_connection->setTooltip(
                    $response->getMeta() ? $response->getMeta()
                                         : _('Response expected')
                );

                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

                $this->_connection->setRequest(
                    $response->getMeta(),
                    11 !== $response->getCode()
                );

            break;

            case 20: // ok

                // Detect MIME type
                switch (true)
                {
                    case $mime = self::getMimeByMeta(
                        $response->getMeta()
                    ): break;

                    case $mime = Filesystem::getMimeByPath(
                        $address->getPath()
                    ): break;

                    case $mime = Filesystem::getMimeByData(
                        $response->getData()
                    ): break;

                    default: $mime = Filesystem::MIME_TEXT_GEMINI;
                }

                // Set MIME
                $this->_connection->setMime(
                    $mime
                );

                // Set title
                $this->_connection->setTitle(
                    $address->getHost()
                );

                // Set subtitle
                $this->_connection->setSubtitle(
                    $response->getMeta()
                );

                // Set tooltip
                $this->_connection->setTooltip(
                    $address->get()
                );

                // Update content data
                $this->_connection->setData(
                    $response->getBody()
                );

            break;

            case 31: // redirect
                     // show link, no follow

                $this->_connection->setTitle(
                    _('Redirect...')
                );

                $this->_connection->setSubtitle(
                    $response->getMeta()
                );

                $this->_connection->setTooltip(
                    sprintf(
                        _('Redirect to %s'),
                        $response->getMeta()
                    )
                );

                $this->_connection->setData(
                    sprintf(
                        '=> %s',
                        $response->getMeta()
                    )
                );

                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

            break;

            default:

                $this->_connection->setTitle(
                    _('Oops!')
                );

                $this->_connection->setSubtitle(
                    sprintf(
                        'Could not open request (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setTooltip(
                    sprintf(
                        'Could not open request (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setData(
                    sprintf(
                        'Could not open request (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );
        }

        $this->_connection->setCompleted(
            true
        );
    }

    public static function getMimeByMeta(
        ?string $meta = null
    ): ?string
    {
        if ($meta)
        {
            preg_match(
                '/(?<mime>([\w]+\/[\w]+))/m',
                $meta,
                $match
            );

            if (isset($match['mime']))
            {
                return $match['mime'];
            }
        }

        return null;
    }
}