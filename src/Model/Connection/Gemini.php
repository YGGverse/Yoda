<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Connection;

use \Yggverse\Yoda\Model\Connection;
use \Yggverse\Yoda\Model\Filesystem;

use \Yggverse\Gemini\Client\Request;
use \Yggverse\Gemini\Client\Response;

use \Yggverse\Net\Address;

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
        int $timeout = 15
    ): void
    {
        // Init request
        $request = new Request(
            $address->get()
        );

        // Get connection settings
        $options = $request->getOptions();

        // Apply identity if available
        if ($identity = $this->matchIdentity($address->get()))
        {
            $crt = tmpfile();

            fwrite(
                $crt,
                $identity->crt
            );

            $options['ssl']['local_cert'] = stream_get_meta_data(
                $crt
            )['uri'];

            $key = tmpfile();

            fwrite(
                $key,
                $identity->key
            );

            $options['ssl']['local_pk'] = stream_get_meta_data(
                $key
            )['uri'];
        }

        // Update connection
        $request->setOptions(
            $options
        );

        // Parse response
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
                    $address->getHost()
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
                    $address->getHost()
                );

                // Set tooltip
                $this->_connection->setTooltip(
                    $address->get()
                );

                // Set data
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
                    $address->getHost()
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

            case 60: // authorization certificate required

                $this->_connection->setAuth(
                    true
                );

                $this->_connection->setTitle(
                    _('Authorization')
                );

                $this->_connection->setSubtitle(
                    $address->getHost()
                );

                $this->_connection->setTooltip(
                    sprintf(
                        'Authorization required (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setData(
                    sprintf(
                        'Authorization required (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

            break;

            case 61: // certificate not authorized

                $this->_connection->setAuth(
                    true
                );

                $this->_connection->setTitle(
                    _('Oops!')
                );

                $this->_connection->setSubtitle(
                    $address->getHost()
                );

                $this->_connection->setTooltip(
                    sprintf(
                        'Authorization certificate not authorized (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setData(
                    sprintf(
                        'Authorization certificate not authorized (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

            break;

            case 62: // certificate not valid

                $this->_connection->setAuth(
                    true
                );

                $this->_connection->setTitle(
                    _('Oops!')
                );

                $this->_connection->setSubtitle(
                    $address->getHost()
                );

                $this->_connection->setTooltip(
                    sprintf(
                        'Authorization certificate not valid (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setData(
                    sprintf(
                        'Authorization certificate not valid (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
                );

                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

            break;

            default:

                // Try cache
                if ($cache = $this->_connection->database->cache->get($address->get()))
                {
                    $this->_connection->setTitle(
                        $cache->title
                    );

                    $this->_connection->setSubtitle(
                        date(
                            'c',
                            $cache->time
                        ) # $cache->subtitle
                    );

                    $this->_connection->setTooltip(
                        $cache->tooltip
                    );

                    $this->_connection->setData(
                        $cache->data
                    );

                    $this->_connection->setMime(
                        $cache->mime
                    );
                }

                else
                {
                    $this->_connection->setTitle(
                        _('Oops!')
                    );

                    $this->_connection->setSubtitle(
                        $address->getHost()
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
        }

        $this->_connection->setCompleted(
            true
        );
    }

    /**
     * Return identity match request | NULL
     *
     * https://geminiprotocol.net/docs/protocol-specification.gmi#client-certificates
     *
     */
    public function matchIdentity(
        string $request,
        array $identities = []
    ): ?object
    {
        foreach ($this->_connection->database->auth->like(sprintf('%s%%', $request)) as $auth)
        {
            $identities[$auth->identity] = $auth->request;
        }

        if ($identities)
        {
            uasort(
                $identities,
                function ($a, $b)
                {
                    return mb_strlen($b) <=> mb_strlen($a);
                }
            );

            return $this->_connection->database->identity->get(
                intval(
                    array_key_first(
                        $identities
                    )
                )
            );
        }

        return null;
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