<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model;

use \Yggverse\Net\Address;

class Connection extends \Yggverse\Yoda\Abstract\Model\Connection
{
    public function request(
        string $request,
        int $timeout = 15
    ): void
    {
        // Build address instance
        $address = new Address(
            $request
        );

        // Detect protocol
        switch ($address->getScheme())
        {
            case 'file':

                (new Connection\File($this))->request(
                    $address
                );

            break;

            case 'gemini': // async

                $pid = pcntl_fork();

                if ($pid === 0)
                {
                    (new Connection\Gemini($this))->request(
                        $address,
                        $timeout
                    );

                    exit;
                }

            break;

            case 'nex': // async

                $pid = pcntl_fork();

                if ($pid === 0)
                {
                    (new Connection\Nex($this))->request(
                        $address,
                        $timeout
                    );

                    exit;
                }

            break;

            case null: // no scheme provided

                // Use gemini protocol by default
                $redirect = new Address(
                    sprintf(
                        'gemini://%s',
                        $address->get()
                    )
                );

                // Hostname valid
                if (filter_var(
                        $redirect->getHost(),
                        FILTER_VALIDATE_DOMAIN,
                        FILTER_FLAG_HOSTNAME
                    )
                ) {
                    // Redirect
                    $this->setRedirect(
                        $redirect->get()
                    );
                }

                // Redirect to default search provider
                else
                {
                    // @TODO custom providers
                    $this->setRedirect(
                        sprintf(
                            'gemini://tlgs.one/search?%s',
                            urlencode(
                                $request
                            )
                        )
                    );
                }

            return;

            default:

                $this->setTitle(
                    _('Oops!')
                );

                $this->setData(
                    _('Protocol not supported')
                );

                $this->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

                $this->setCompleted(
                    true
                );
        }
    }
}