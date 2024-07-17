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

        // Route status code
        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
        switch ($response->getCode())
        {
            case 10: // response expected
            case 11: // sensitive input

                $this->_connection->setMime(
                    $this->_connection::MIME_TEXT_GEMINI
                );

                $this->_connection->setRequest(
                    $response->getMeta(),
                    11 !== $response->getCode()
                );

            break;

            case 20: // ok

                // Update content data
                $this->_connection->setData(
                    $response->getBody()
                );

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

                    default: $mime = null;
                }

                $this->_connection->setMime(
                    $mime
                );

            break;

            case 31: // redirect
                     // show link, no follow

                $this->_connection->setTitle(
                    _('Redirect!')
                );

                $this->_connection->setData(
                    sprintf(
                        '=> %s',
                        $response->getMeta()
                    )
                );

            break;

            default:

                $this->_connection->setTitle(
                    _('Oops!')
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
                    $this->_connection::MIME_TEXT_GEMINI
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